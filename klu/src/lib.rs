use std::alloc::Layout;
use std::cell::Cell;
use std::marker::PhantomData;
use std::ops::Index;
use std::ptr::{self, NonNull};
use std::rc::Rc;

pub use raw::{KluData, KluIndex};

mod raw;
#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct KluSettings<I: KluIndex> {
    data: Box<I::KluCommon>,
}

impl<I: KluIndex> KluSettings<I> {
    pub fn new() -> Self {
        unsafe {
            let raw = std::alloc::alloc(Layout::new::<I::KluCommon>()) as *mut I::KluCommon;
            I::klu_defaults(raw);
            Self {
                data: Box::from_raw(raw),
            }
        }
    }

    pub fn as_ffi(&self) -> *mut I::KluCommon {
        self.data.as_ref() as *const I::KluCommon as *mut I::KluCommon
    }

    pub fn check_status(&self) {
        I::check_status(&self.data)
    }

    pub fn is_singular(&self) -> bool {
        I::is_singular(&self.data)
    }

    pub fn get_rcond(&self) -> f64 {
        I::get_rcond(&self.data)
    }
}

impl<I: KluIndex> Default for KluSettings<I> {
    fn default() -> Self {
        Self::new()
    }
}

/// A compressed column form SparsMatrix whose shape is fixed
pub struct FixedKluMatrix<I: KluIndex, D: KluData> {
    pub spec: Rc<KluMatrixSpec<I>>,
    data: NonNull<[Cell<D>]>,
    klu_numeric: Option<NonNull<I::KluNumeric>>,
}

impl<I: KluIndex, D: KluData> FixedKluMatrix<I, D> {
    /// Constructs a new matrix for the provided [`KluMatrixSpec<I>`] by allocating space where the
    /// data can be stored
    ///
    /// # Returns
    ///
    /// the constructed matrix if the matrix is not empty.
    /// If the matrix is empty `None` is retruned instead
    pub fn new(spec: Rc<KluMatrixSpec<I>>) -> Option<Self> {
        if spec.row_indices.is_empty() {
            return None;
        }

        let data = vec![Cell::new(D::default()); spec.row_indices.len()];
        let data = Box::into_raw(data.into_boxed_slice());
        let data = NonNull::new(data).expect("Box::into_raw never returns null");

        Some(Self {
            spec,
            data,
            klu_numeric: None,
        })
    }

    pub fn data(&self) -> &[Cell<D>] {
        // this is save because FrozenSparseMatrix makes the API guarantee that `self.data` is
        // always a valid owned pointer constructed from a `Box`
        unsafe { self.data.as_ref() }
    }

    /// Returns a pointer to the matrix data
    ///
    /// # Safety
    ///
    /// The returned pointer must **never be dereferenced**.
    /// Turing this data into any reference always causes undefined behaviour
    ///
    /// This pointer point to data inside an `UnsafeCell` so you should use `std::ptr` methods to
    /// access it or turn it into `&Cell<D>` or `&UnsafeCell<D>`
    pub fn data_ptr(&self) -> *mut D {
        self.data()[0].as_ptr()
    }

    pub fn write_all(&self, val: D) {
        unsafe { ptr::copy_nonoverlapping(&val, self.data_ptr(), self.data.len()) }
    }

    /// Perform lu_factorization of the matrix using KLU
    /// If the matrix was already factorized previously and `refactor_threshold` is given, it is
    /// first attempted to refactorize the matrix. If this fails (either due to an KLU error or
    /// because the rcond is larger than the provided threshold) full factorization is preformed.
    ///
    /// Calling to funciton is a prerequisite to calling [`solve_linear_system`]
    pub fn lu_factorize(&mut self, refactor_threshold: Option<f64>) -> bool {
        match (self.klu_numeric, refactor_threshold) {
            (Some(klu_numeric), None) => unsafe {
                D::klu_free_numeric::<I>(&mut klu_numeric.as_ptr(), self.spec.settings.as_ffi())
            },
            (Some(klu_numeric), Some(rcond_threshold)) => {
                let res = unsafe {
                    D::klu_refactor(
                        // KLU does not modify these values they only need to be mut bceuase C has no concept of a const pointer
                        self.spec.column_offsets.as_ptr(),
                        self.spec.row_indices.as_ptr(),
                        self.data_ptr(),
                        self.spec.klu_symbolic.as_ptr(),
                        klu_numeric.as_ptr(),
                        self.spec.settings.as_ffi(),
                    ) && D::klu_rcond::<I>(
                        self.spec.klu_symbolic.as_ptr(),
                        klu_numeric.as_ptr(),
                        self.spec.settings.as_ffi(),
                    )
                };
                self.spec.settings.check_status();
                if !self.spec.settings.is_singular()
                    && self.spec.settings.get_rcond() <= rcond_threshold
                {
                    // refactoring succeded we are done here
                    assert!(res, "KLU produced unkown error");
                    return false;
                }

                unsafe {
                    D::klu_free_numeric::<I>(&mut klu_numeric.as_ptr(), self.spec.settings.as_ffi())
                }
                self.klu_numeric = None;
            }

            _ => (),
        };

        let klu_numeric = unsafe {
            D::klu_factor(
                // KLU does not modify these values they only need to be mut because C has not concept of a const pointer
                self.spec.column_offsets.as_ptr(),
                self.spec.row_indices.as_ptr(),
                self.data_ptr(),
                self.spec.klu_symbolic.as_ptr(),
                self.spec.settings.as_ffi(),
            )
        };
        self.spec.settings.check_status();
        if self.spec.settings.is_singular() {
            return true;
        }

        let klu_numeric = NonNull::new(klu_numeric).expect("KLU retruned a valid numeric object");
        self.klu_numeric = Some(klu_numeric);
        false
    }

    /// solves the linear system `Ax=b`. The `b` vector is read from `rhs` at the beginning of the
    /// function. After the functin completes `x` was written into `x`
    ///
    /// **Note**: This function assumes that [`lu_factorize`] was called first.
    /// If this is not the case this functions panics.
    pub fn solve_linear_system(&self, rhs: &mut [D]) {
        // TODO allow solving multiple rhs

        let klu_numeric = self
            .klu_numeric
            .expect("factorize must be called before solve");
        let res = unsafe {
            D::klu_solve::<I>(
                self.spec.klu_symbolic.as_ptr(),
                klu_numeric.as_ptr(),
                I::from_usize(rhs.len()),
                I::from_usize(1),
                rhs.as_mut_ptr(),
                self.spec.settings.as_ffi(),
            )
        };

        self.spec.settings.check_status();

        assert!(res, "KLU produced unkown error");
    }
}

impl<I: KluIndex, D: KluData> Index<usize> for FixedKluMatrix<I, D> {
    type Output = Cell<D>;

    fn index(&self, i: usize) -> &Self::Output {
        &self.data()[i]
    }
}

impl<I: KluIndex, D: KluData> Index<(I, I)> for FixedKluMatrix<I, D> {
    type Output = Cell<D>;

    fn index(&self, (column, row): (I, I)) -> &Self::Output {
        let offset = self.spec.offset(column, row);
        &self[offset]
    }
}

impl<I: KluIndex, D: KluData> Drop for FixedKluMatrix<I, D> {
    fn drop(&mut self) {
        if let Some(klu_numeric) = self.klu_numeric.take() {
            unsafe {
                D::klu_free_numeric::<I>(&mut klu_numeric.as_ptr(), self.spec.settings.as_ffi())
            }
        }

        unsafe {
            Box::from_raw(self.data.as_ptr());
        }
    }
}

#[derive(Debug)]
pub struct KluMatrixSpec<I: KluIndex> {
    column_offsets: Box<[I]>,
    row_indices: Box<[I]>,
    settings: KluSettings<I>,
    klu_symbolic: NonNull<I::KluSymbolic>,
    pd: PhantomData<I::KluSymbolic>,
}

impl<I: KluIndex> KluMatrixSpec<I> {
    pub fn entry_cnt(&self) -> usize {
        self.row_indices.len()
    }

    pub fn new(columns: &[Vec<I>], klu_settings: KluSettings<I>) -> Rc<Self> {
        let mut column_offsets = Vec::with_capacity(columns.len() + 1);
        column_offsets.push(I::from_usize(0));
        let mut num_entries = 0;
        column_offsets.extend(columns.iter().map(|colum| {
            num_entries += colum.len();
            I::from_usize(num_entries)
        }));

        let num_cols = I::from_usize(columns.len());
        let mut column_offsets = column_offsets.into_boxed_slice();
        let mut row_indices = Vec::with_capacity(num_entries);

        for colmun in columns {
            row_indices.extend_from_slice(colmun)
        }

        let klu_symbolic = unsafe {
            I::klu_analyze(
                num_cols,
                column_offsets.as_mut_ptr(),
                row_indices.as_mut_ptr(),
                klu_settings.as_ffi(),
            )
        };

        klu_settings.check_status();

        let res = Self {
            column_offsets,
            row_indices: row_indices.into_boxed_slice(),
            klu_symbolic: NonNull::new(klu_symbolic).unwrap(),
            settings: klu_settings,
            pd: PhantomData,
        };
        Rc::new(res)
    }

    pub fn create_matrix<D: KluData>(self: Rc<Self>) -> Option<FixedKluMatrix<I, D>> {
        FixedKluMatrix::new(self)
    }

    pub fn offset(&self, column: I, row: I) -> usize {
        let column = column.into_usize();
        let end = self.column_offsets[column + 1].into_usize();

        let column_offset = self.column_offsets[column].into_usize();

        let pos = self.row_indices[column_offset..end]
            .iter()
            .position(|&r| r == row)
            .unwrap();
        column_offset + pos
    }
}

impl<I: KluIndex> Drop for KluMatrixSpec<I> {
    fn drop(&mut self) {
        unsafe { I::klu_free_symbolic(&mut self.klu_symbolic.as_ptr(), self.settings.as_ffi()) }
    }
}

pub struct KluMatrixBuilder<I: KluIndex>(Vec<Vec<I>>);

impl<I: KluIndex> KluMatrixBuilder<I> {
    pub fn new(dim: I) -> Self {
        Self(vec![Vec::with_capacity(64); dim.into_usize()])
    }

    pub fn add_entry(&mut self, column: I, row: I) {
        if self.0.len() < column.into_usize() {
            self.0
                .resize_with(column.into_usize(), || Vec::with_capacity(64))
        }
        let column = &mut self.0[column.into_usize()];
        // Keep  the set unique and sorted (the latter is not necessary but makes insert fast and depending on KLU handles this be a nice property later)
        let dst = column.partition_point(|it| *it < row);
        if column.get(dst).map_or(true, |&it| it != row) {
            column.insert(dst, row)
        }
    }

    pub fn finish(self, klu_settings: KluSettings<I>) -> Rc<KluMatrixSpec<I>> {
        KluMatrixSpec::new(&self.0, klu_settings)
    }
}
