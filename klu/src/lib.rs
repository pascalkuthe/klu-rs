use std::alloc::Layout;
use std::cell::Cell;
use std::marker::PhantomData;
use std::mem;
use std::ops::Index;
use std::ptr::NonNull;
use std::rc::Rc;

pub use raw::{KluData, KluIndex};

mod raw;
#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct KluSettings<I: KluIndex> {
    data: NonNull<I::KluCommon>,
}

impl<I: KluIndex> KluSettings<I> {
    pub fn new() -> Self {
        unsafe {
            let raw = std::alloc::alloc(Layout::new::<I::KluCommon>()) as *mut I::KluCommon;
            I::klu_defaults(raw);
            Self {
                data: NonNull::new_unchecked(raw),
            }
        }
    }

    pub fn as_ffi(&self) -> *mut I::KluCommon {
        self.data.as_ptr()
    }

    pub fn check_status(&self) {
        I::check_status(unsafe { self.data.as_ref() })
    }

    pub fn is_singular(&self) -> bool {
        I::is_singular(unsafe { self.data.as_ref() })
    }

    pub fn get_rcond(&self) -> f64 {
        I::get_rcond(unsafe { self.data.as_ref() })
    }
}

impl<I: KluIndex> Drop for KluSettings<I> {
    fn drop(&mut self) {
        unsafe { std::alloc::dealloc(self.as_ffi() as *mut u8, Layout::new::<I::KluCommon>()) }
    }
}

impl<I: KluIndex> Default for KluSettings<I> {
    fn default() -> Self {
        Self::new()
    }
}

/// A compressed column form SparsMatrix whose shape is fixed
pub struct FixedKluMatrix<I: KluIndex, D: KluData> {
    spec: Rc<KluMatrixSpec<I>>,
    data: Option<NonNull<[D]>>,
    klu_numeric: Option<NonNull<I::KluNumeric>>,
}

impl<I: KluIndex, D: KluData> FixedKluMatrix<I, D> {
    /// Obtain the allocation of the matrix data
    /// This function can be used with [`from_raw`] and [`KluMatrixSpec::reinit`] to reuse a matrix
    /// allocation.
    ///
    /// # Safety
    ///
    /// This function invalidates any pointers into the matrix
    pub fn into_alloc(mut self) -> Vec<D> {
        let klu_numeric = self.klu_numeric.take();
        self.free_numeric(klu_numeric);
        // # SAFETY: This is save because data was constructed from a leaked box
        unsafe {
            let data = self.data.take().unwrap().as_mut();
            Box::from_raw(data).into()
        }
    }

    /// Constructs a new matrix for the provided [`KluMatrixSpec<I>`].
    /// `alloc` is used to store the data. If not enough space is available `alloc` is resized as
    /// required.
    ///
    /// The matrix is always intially filled with zeros no matter the previous content of `alloc`
    ///
    /// # Returns
    ///
    /// the constructed matrix if the matrix is not empty.
    /// If the matrix is empty `None` is retruned instead
    pub fn new_with_alloc(spec: Rc<KluMatrixSpec<I>>, mut alloc: Vec<D>) -> Option<Self> {
        if spec.row_indices.is_empty() {
            return None;
        }

        alloc.fill(D::default());
        alloc.resize(spec.row_indices.len(), D::default());
        // Safety: this is save because while data and alloc might alias they are raw pointers so this is allowed
        // this construct might seem a bit odd because we are storing the same pointer twice
        // The reason for this construct is that while *mut Cell<T> == *mut T holds true in theory, this is outside of rusts stability garuntees
        // However
        let data = Box::leak(alloc.into_boxed_slice());

        Some(Self {
            spec,
            data: Some(data.into()),
            klu_numeric: None,
        })
    }

    /// Constructs a new matrix for the provided [`KluMatrixSpec<I>`] by allocating space where the
    /// data can be stored.
    ///
    /// The matrix is intially filled with zeros.
    ///
    /// # Returns
    ///
    /// the constructed matrix if the matrix is not empty.
    /// If the matrix is empty `None` is retruned instead
    pub fn new(spec: Rc<KluMatrixSpec<I>>) -> Option<Self> {
        Self::new_with_alloc(spec, Vec::new())
    }

    pub fn data(&self) -> &[Cell<D>] {
        // # Safety self.data is constructed from a mutable pointer so there are no references to the data directly not constructed trough a cell
        unsafe { &*(self.data.unwrap().as_ptr() as *const [Cell<D>]) }
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
        self.data.unwrap().as_ptr() as *mut D
    }

    pub fn write_all(&self, val: D) {
        for entry in self.data() {
            entry.set(val)
        }
    }

    pub fn write_zero(&self) {
        // Safety: the sealed KLU data trait is only implemented for types (f64, Complex64) were byte zero is valid and equal to algebraic zero
        unsafe { self.data_ptr().write_bytes(0, self.data().len()) }
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
    /// function. After the functin completes `x` was written into `rhs`
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

    /// solves the linear system `A^T x=b` The `b` vector is read from `rhs` at the beginning of the
    /// function. After the functin completes `x` was written into `rhs`
    ///
    /// **Note**: This function assumes that [`lu_factorize`] was called first.
    /// If this is not the case this functions panics.
    pub fn solve_linear_tranose_system(&self, rhs: &mut [D]) {
        // TODO allow solving multiple rhs

        let klu_numeric = self
            .klu_numeric
            .expect("factorize must be called before solve");
        let res = unsafe {
            D::klu_tsolve::<I>(
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
    fn free_numeric(&self, klu_numeric: Option<NonNull<I::KluNumeric>>) {
        if let Some(klu_numeric) = klu_numeric {
            unsafe {
                D::klu_free_numeric::<I>(&mut klu_numeric.as_ptr(), self.spec.settings.as_ffi())
            }
        }
    }
    pub fn get(&self, column: I, row: I) -> Option<&Cell<D>> {
        let offset = self.spec.offset(column, row)?;
        Some(&self[offset])
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
        self.get(column, row).unwrap()
    }
}

impl<I: KluIndex, D: KluData> Drop for FixedKluMatrix<I, D> {
    fn drop(&mut self) {
        let klu_numeric = self.klu_numeric.take();
        self.free_numeric(klu_numeric);

        if let Some(data) = self.data.take() {
            unsafe {
                drop(Box::from_raw(data.as_ptr()));
            }
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

    /// Constructs a new matrix specification by reusing the allocations within this spec.
    /// See [`new`] for details
    pub fn reinit(&mut self, columns: &[Vec<I>]) {
        self.free_symbolic();
        self.init(columns)
    }

    fn init(&mut self, columns: &[Vec<I>]) {
        let mut column_offsets: Vec<_> =
            mem::replace(&mut self.column_offsets, Box::new([])).into();
        column_offsets.clear();

        column_offsets.reserve(columns.len() + 1);
        column_offsets.push(I::from_usize(0));
        let mut num_entries = 0;
        column_offsets.extend(columns.iter().map(|colum| {
            num_entries += colum.len();
            I::from_usize(num_entries)
        }));

        let num_cols = I::from_usize(columns.len());
        let mut row_indices: Vec<_> = mem::replace(&mut self.row_indices, Box::new([])).into();
        row_indices.clear();
        row_indices.reserve(num_entries);

        for colmun in columns {
            row_indices.extend_from_slice(colmun)
        }

        let klu_symbolic = unsafe {
            I::klu_analyze(
                num_cols,
                column_offsets.as_mut_ptr(),
                row_indices.as_mut_ptr(),
                self.settings.as_ffi(),
            )
        };

        self.settings.check_status();
        self.klu_symbolic = NonNull::new(klu_symbolic)
            .expect("klu_analyze returns a non null pointer if the status is ok");
        self.column_offsets = column_offsets.into_boxed_slice();
        self.row_indices = row_indices.into_boxed_slice();
    }

    /// Constructs a new matrix spec from a column sparse matrix description.
    pub fn new(columns: &[Vec<I>], klu_settings: KluSettings<I>) -> Rc<Self> {
        let mut res = Self {
            column_offsets: Box::new([]),
            row_indices: Box::new([]),
            klu_symbolic: NonNull::dangling(),
            settings: klu_settings,
            pd: PhantomData,
        };
        res.init(columns);
        Rc::new(res)
    }

    pub fn create_matrix<D: KluData>(self: Rc<Self>) -> Option<FixedKluMatrix<I, D>> {
        FixedKluMatrix::new(self)
    }

    pub fn offset(&self, column: I, row: I) -> Option<usize> {
        let column = column.into_usize();
        let end = self.column_offsets[column + 1].into_usize();

        let column_offset = self.column_offsets[column].into_usize();

        let pos = self.row_indices[column_offset..end]
            .iter()
            .position(|&r| r == row)?;
        Some(column_offset + pos)
    }

    fn free_symbolic(&self) {
        unsafe { I::klu_free_symbolic(&mut self.klu_symbolic.as_ptr(), self.settings.as_ffi()) }
    }
}

impl<I: KluIndex> Drop for KluMatrixSpec<I> {
    fn drop(&mut self) {
        self.free_symbolic()
    }
}

pub struct KluMatrixBuilder<I: KluIndex> {
    columns: Vec<Vec<I>>,
    dim: I,
}

impl<I: KluIndex> KluMatrixBuilder<I> {
    pub fn reset(&mut self, dim: I) {
        self.ensure_dim(dim);
        for column in &mut self.columns {
            column.clear();
        }
    }

    pub fn new(dim: I) -> Self {
        Self {
            columns: vec![Vec::with_capacity(64); dim.into_usize()],
            dim,
        }
    }

    fn ensure_dim(&mut self, dim: I) {
        if self.dim < dim {
            self.columns
                .resize_with(dim.into_usize(), || Vec::with_capacity(64));
            self.dim = dim;
        }
    }

    pub fn add_entry(&mut self, column: I, row: I) {
        self.ensure_dim(column + I::from_usize(1));
        let column = &mut self.columns[column.into_usize()];
        // Keep  the set unique and sorted (the latter is not necessary but makes insert fast and depending on KLU handles this be a nice property later)
        let dst = column.partition_point(|it| *it < row);
        if column.get(dst).map_or(true, |&it| it != row) {
            column.insert(dst, row)
        }
    }

    pub fn columns(&self) -> &[Vec<I>] {
        &self.columns[..self.dim.into_usize()]
    }

    pub fn finish(&self, klu_settings: KluSettings<I>) -> Rc<KluMatrixSpec<I>> {
        KluMatrixSpec::new(self.columns(), klu_settings)
    }

    pub fn reinit(&self, spec: &mut KluMatrixSpec<I>) {
        spec.reinit(self.columns())
    }
}
