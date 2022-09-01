use klu_sys::{
    klu_analyze, klu_defaults, klu_factor, klu_free_numeric, klu_free_symbolic, klu_l_analyze,
    klu_l_defaults, klu_l_factor, klu_l_free_numeric, klu_l_free_symbolic, klu_l_rcond,
    klu_l_refactor, klu_l_solve, klu_l_tsolve, klu_rcond, klu_refactor, klu_solve, klu_tsolve,
    klu_z_factor, klu_z_free_numeric, klu_z_rcond, klu_z_refactor, klu_z_solve, klu_z_tsolve,
    klu_zl_factor, klu_zl_free_numeric, klu_zl_rcond, klu_zl_refactor, klu_zl_solve, klu_zl_tsolve,
    KluCommon, KluLCommon, KluLNumeric, KluLSymbolic, KluNumeric, KluSymbolic,
};
use num_complex::{Complex64, ComplexFloat};

use crate::raw::sealed::Sealed;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, Mul, Sub};

mod sealed {
    use num_complex::Complex64;

    pub trait Sealed {}
    impl Sealed for f64 {}
    impl Sealed for Complex64 {}
    impl Sealed for i32 {}
    impl Sealed for i64 {}
}

/// Values that can be used by the KLU solver.
/// The functions of this trait are all unsafe because they directly call the underlying C implementation.
#[allow(clippy::missing_safety_doc)]
pub trait KluData:
    Sealed
    + Copy
    + PartialEq
    + Debug
    + Default
    + Mul
    + Sub
    + Add
    + AddAssign
    + Div
    + ComplexFloat<Real = f64>
{
    unsafe fn klu_solve<I: KluIndex>(
        symbolic: *mut I::KluSymbolic,
        numeric: *mut I::KluNumeric,
        rhs_dimension: I,
        number_rhs: I,
        rhs_data: *mut Self,
        common: *mut I::KluCommon,
    ) -> bool;

    unsafe fn klu_tsolve<I: KluIndex>(
        symbolic: *mut I::KluSymbolic,
        numeric: *mut I::KluNumeric,
        rhs_dimension: I,
        number_rhs: I,
        rhs_data: *mut Self,
        common: *mut I::KluCommon,
    ) -> bool;

    unsafe fn klu_factor<I: KluIndex>(
        colum_offsets: *const I,
        row_indices: *const I,
        data: *mut Self,
        symbolic: *mut I::KluSymbolic,
        common: *mut I::KluCommon,
    ) -> *mut I::KluNumeric;

    unsafe fn klu_refactor<I: KluIndex>(
        colum_offsets: *const I,
        row_indices: *const I,
        data: *mut Self,
        symbolic: *mut I::KluSymbolic,
        numeric: *mut I::KluNumeric,
        common: *mut I::KluCommon,
    ) -> bool;

    unsafe fn klu_free_numeric<I: KluIndex>(
        numeric: *mut *mut I::KluNumeric,
        common: *mut I::KluCommon,
    );

    unsafe fn klu_rcond<I: KluIndex>(
        symbolic: *mut I::KluSymbolic,
        numeric: *mut I::KluNumeric,
        common: *mut I::KluCommon,
    ) -> bool;
}

/// Values that can be used by the KLU solver.
/// The functions of this trait are all unsafe because they directly call the underlying C implementation.
#[allow(clippy::missing_safety_doc)]
pub trait KluIndex:
    PartialEq + PartialOrd + Ord + Debug + Copy + Clone + Eq + Sealed + Add<Output = Self>
{
    type KluCommon: Debug;
    type KluNumeric: Debug;
    type KluSymbolic: Debug;

    fn from_usize(val: usize) -> Self;
    fn into_usize(self) -> usize;

    unsafe fn klu_defaults(common: *mut Self::KluCommon) -> Self;

    unsafe fn klu_analyze(
        dim: Self,
        colum_offsets: *const Self,
        row_indices: *const Self,
        common: *mut Self::KluCommon,
    ) -> *mut Self::KluSymbolic;

    unsafe fn klu_free_symbolic(
        symbolic: *mut *mut Self::KluSymbolic,
        common: *mut Self::KluCommon,
    );

    unsafe fn klu_factor(
        colum_offsets: *const Self,
        row_indices: *const Self,
        data: *mut f64,
        symbolic: *mut Self::KluSymbolic,
        common: *mut Self::KluCommon,
    ) -> *mut Self::KluNumeric;

    unsafe fn klu_z_factor(
        colum_offsets: *const Self,
        row_indices: *const Self,
        data: *mut f64,
        symbolic: *mut Self::KluSymbolic,
        common: *mut Self::KluCommon,
    ) -> *mut Self::KluNumeric;

    unsafe fn klu_refactor(
        colum_offsets: *const Self,
        row_indices: *const Self,
        data: *mut f64,
        symbolic: *mut Self::KluSymbolic,
        numeric: *mut Self::KluNumeric,
        common: *mut Self::KluCommon,
    ) -> bool;

    unsafe fn klu_z_refactor(
        colum_offsets: *const Self,
        row_indices: *const Self,
        data: *mut f64,
        symbolic: *mut Self::KluSymbolic,
        numeric: *mut Self::KluNumeric,
        common: *mut Self::KluCommon,
    ) -> bool;

    unsafe fn klu_solve(
        symbolic: *mut Self::KluSymbolic,
        numeric: *mut Self::KluNumeric,
        rhs_dimension: Self,
        number_rhs: Self,
        rhs_data: *mut f64,
        common: *mut Self::KluCommon,
    ) -> bool;

    unsafe fn klu_tsolve(
        symbolic: *mut Self::KluSymbolic,
        numeric: *mut Self::KluNumeric,
        rhs_dimension: Self,
        number_rhs: Self,
        rhs_data: *mut f64,
        common: *mut Self::KluCommon,
    ) -> bool;

    unsafe fn klu_z_solve(
        symbolic: *mut Self::KluSymbolic,
        numeric: *mut Self::KluNumeric,
        rhs_dimension: Self,
        number_rhs: Self,
        rhs_data: *mut f64,
        common: *mut Self::KluCommon,
    ) -> bool;

    unsafe fn klu_z_tsolve(
        symbolic: *mut Self::KluSymbolic,
        numeric: *mut Self::KluNumeric,
        rhs_dimension: Self,
        number_rhs: Self,
        rhs_data: *mut f64,
        common: *mut Self::KluCommon,
    ) -> bool;

    unsafe fn klu_free_numeric(numeric: *mut *mut Self::KluNumeric, common: *mut Self::KluCommon);

    unsafe fn klu_z_free_numeric(numeric: *mut *mut Self::KluNumeric, common: *mut Self::KluCommon);

    fn check_status(common: &Self::KluCommon);
    fn get_rcond(common: &Self::KluCommon) -> f64;
    fn is_singular(common: &Self::KluCommon) -> bool;

    unsafe fn klu_rcond(
        symbolic: *mut Self::KluSymbolic,
        numeric: *mut Self::KluNumeric,
        common: *mut Self::KluCommon,
    ) -> bool;

    unsafe fn klu_z_rcond(
        symbolic: *mut Self::KluSymbolic,
        numeric: *mut Self::KluNumeric,
        common: *mut Self::KluCommon,
    ) -> bool;
}

impl KluData for f64 {
    unsafe fn klu_solve<I: KluIndex>(
        symbolic: *mut <I as KluIndex>::KluSymbolic,
        numeric: *mut <I as KluIndex>::KluNumeric,
        rhs_dimension: I,
        number_rhs: I,
        rhs_data: *mut Self,
        common: *mut <I as KluIndex>::KluCommon,
    ) -> bool {
        I::klu_solve(
            symbolic,
            numeric,
            rhs_dimension,
            number_rhs,
            rhs_data,
            common,
        )
    }

    unsafe fn klu_tsolve<I: KluIndex>(
        symbolic: *mut I::KluSymbolic,
        numeric: *mut I::KluNumeric,
        rhs_dimension: I,
        number_rhs: I,
        rhs_data: *mut Self,
        common: *mut I::KluCommon,
    ) -> bool {
        I::klu_tsolve(
            symbolic,
            numeric,
            rhs_dimension,
            number_rhs,
            rhs_data,
            common,
        )
    }

    unsafe fn klu_factor<I: KluIndex>(
        colum_offsets: *const I,
        row_indices: *const I,
        data: *mut Self,
        symbolic: *mut I::KluSymbolic,
        common: *mut I::KluCommon,
    ) -> *mut I::KluNumeric {
        I::klu_factor(colum_offsets, row_indices, data, symbolic, common)
    }

    unsafe fn klu_refactor<I: KluIndex>(
        colum_offsets: *const I,
        row_indices: *const I,
        data: *mut Self,
        symbolic: *mut I::KluSymbolic,
        numeric: *mut I::KluNumeric,
        common: *mut I::KluCommon,
    ) -> bool {
        I::klu_refactor(colum_offsets, row_indices, data, symbolic, numeric, common)
    }

    unsafe fn klu_free_numeric<I: KluIndex>(
        numeric: *mut *mut I::KluNumeric,
        common: *mut I::KluCommon,
    ) {
        I::klu_free_numeric(numeric, common)
    }

    unsafe fn klu_rcond<I: KluIndex>(
        symbolic: *mut I::KluSymbolic,
        numeric: *mut I::KluNumeric,
        common: *mut I::KluCommon,
    ) -> bool {
        I::klu_rcond(symbolic, numeric, common)
    }
}

impl KluData for Complex64 {
    unsafe fn klu_solve<I: KluIndex>(
        symbolic: *mut <I as KluIndex>::KluSymbolic,
        numeric: *mut <I as KluIndex>::KluNumeric,
        rhs_dimension: I,
        number_rhs: I,
        rhs_data: *mut Self,
        common: *mut <I as KluIndex>::KluCommon,
    ) -> bool {
        I::klu_z_solve(
            symbolic,
            numeric,
            rhs_dimension,
            number_rhs,
            rhs_data as *mut f64,
            common,
        )
    }

    unsafe fn klu_tsolve<I: KluIndex>(
        symbolic: *mut I::KluSymbolic,
        numeric: *mut I::KluNumeric,
        rhs_dimension: I,
        number_rhs: I,
        rhs_data: *mut Self,
        common: *mut I::KluCommon,
    ) -> bool {
        I::klu_z_tsolve(
            symbolic,
            numeric,
            rhs_dimension,
            number_rhs,
            rhs_data as *mut f64,
            common,
        )
    }

    unsafe fn klu_factor<I: KluIndex>(
        colum_offsets: *const I,
        row_indices: *const I,
        data: *mut Self,
        symbolic: *mut I::KluSymbolic,
        common: *mut I::KluCommon,
    ) -> *mut I::KluNumeric {
        I::klu_z_factor(
            colum_offsets,
            row_indices,
            data as *mut f64,
            symbolic,
            common,
        )
    }

    unsafe fn klu_refactor<I: KluIndex>(
        colum_offsets: *const I,
        row_indices: *const I,
        data: *mut Self,
        symbolic: *mut I::KluSymbolic,
        numeric: *mut I::KluNumeric,
        common: *mut I::KluCommon,
    ) -> bool {
        I::klu_z_refactor(
            colum_offsets,
            row_indices,
            data as *mut f64,
            symbolic,
            numeric,
            common,
        )
    }

    unsafe fn klu_free_numeric<I: KluIndex>(
        numeric: *mut *mut I::KluNumeric,
        common: *mut I::KluCommon,
    ) {
        I::klu_z_free_numeric(numeric, common)
    }

    unsafe fn klu_rcond<I: KluIndex>(
        symbolic: *mut I::KluSymbolic,
        numeric: *mut I::KluNumeric,
        common: *mut I::KluCommon,
    ) -> bool {
        I::klu_z_rcond(symbolic, numeric, common)
    }
}
// targets where c_int != i32 are not supported
impl KluIndex for i32 {
    type KluCommon = KluCommon;
    type KluNumeric = KluNumeric;
    type KluSymbolic = KluSymbolic;

    fn from_usize(val: usize) -> Self {
        debug_assert!(val <= Self::MAX as usize);
        val as Self
    }

    fn into_usize(self) -> usize {
        debug_assert!(self >= 0);
        self as usize
    }

    unsafe fn klu_defaults(common: *mut Self::KluCommon) -> Self {
        klu_defaults(common)
    }

    unsafe fn klu_analyze(
        dim: Self,
        colum_offsets: *const Self,
        row_indices: *const Self,
        common: *mut Self::KluCommon,
    ) -> *mut Self::KluSymbolic {
        klu_analyze(
            dim,
            colum_offsets as *mut Self,
            row_indices as *mut Self,
            common,
        )
    }

    unsafe fn klu_free_symbolic(
        symbolic: *mut *mut Self::KluSymbolic,
        common: *mut Self::KluCommon,
    ) {
        if klu_free_symbolic(symbolic, common) == 0 {
            unreachable!("freeing klu symbolic object failed")
        }
    }

    unsafe fn klu_factor(
        colum_offsets: *const Self,
        row_indices: *const Self,
        data: *mut f64,
        symbolic: *mut Self::KluSymbolic,
        common: *mut Self::KluCommon,
    ) -> *mut Self::KluNumeric {
        klu_factor(
            colum_offsets as *mut Self,
            row_indices as *mut Self,
            data,
            symbolic,
            common,
        )
    }

    unsafe fn klu_z_factor(
        colum_offsets: *const Self,
        row_indices: *const Self,
        data: *mut f64,
        symbolic: *mut Self::KluSymbolic,
        common: *mut Self::KluCommon,
    ) -> *mut Self::KluNumeric {
        klu_z_factor(
            colum_offsets as *mut Self,
            row_indices as *mut Self,
            data,
            symbolic,
            common,
        )
    }

    unsafe fn klu_refactor(
        colum_offsets: *const Self,
        row_indices: *const Self,
        data: *mut f64,
        symbolic: *mut Self::KluSymbolic,
        numeric: *mut Self::KluNumeric,
        common: *mut Self::KluCommon,
    ) -> bool {
        klu_refactor(
            colum_offsets as *mut Self,
            row_indices as *mut Self,
            data,
            symbolic,
            numeric,
            common,
        ) != 0
    }

    unsafe fn klu_z_refactor(
        colum_offsets: *const Self,
        row_indices: *const Self,
        data: *mut f64,
        symbolic: *mut Self::KluSymbolic,
        numeric: *mut Self::KluNumeric,
        common: *mut Self::KluCommon,
    ) -> bool {
        klu_z_refactor(
            colum_offsets as *mut Self,
            row_indices as *mut Self,
            data,
            symbolic,
            numeric,
            common,
        ) != 0
    }

    unsafe fn klu_solve(
        symbolic: *mut Self::KluSymbolic,
        numeric: *mut Self::KluNumeric,
        rhs_dimension: Self,
        number_rhs: Self,
        rhs_data: *mut f64,
        common: *mut Self::KluCommon,
    ) -> bool {
        klu_solve(
            symbolic,
            numeric,
            rhs_dimension,
            number_rhs,
            rhs_data,
            common,
        ) != 0
    }

    unsafe fn klu_tsolve(
        symbolic: *mut Self::KluSymbolic,
        numeric: *mut Self::KluNumeric,
        rhs_dimension: Self,
        number_rhs: Self,
        rhs_data: *mut f64,
        common: *mut Self::KluCommon,
    ) -> bool {
        klu_tsolve(
            symbolic,
            numeric,
            rhs_dimension,
            number_rhs,
            rhs_data,
            common,
        ) != 0
    }

    unsafe fn klu_z_solve(
        symbolic: *mut Self::KluSymbolic,
        numeric: *mut Self::KluNumeric,
        rhs_dimension: Self,
        number_rhs: Self,
        rhs_data: *mut f64,
        common: *mut Self::KluCommon,
    ) -> bool {
        klu_z_solve(
            symbolic,
            numeric,
            rhs_dimension,
            number_rhs,
            rhs_data,
            common,
        ) != 0
    }

    unsafe fn klu_z_tsolve(
        symbolic: *mut Self::KluSymbolic,
        numeric: *mut Self::KluNumeric,
        rhs_dimension: Self,
        number_rhs: Self,
        rhs_data: *mut f64,
        common: *mut Self::KluCommon,
    ) -> bool {
        klu_z_tsolve(
            symbolic,
            numeric,
            rhs_dimension,
            number_rhs,
            rhs_data,
            0,
            common,
        ) != 0
    }

    unsafe fn klu_free_numeric(numeric: *mut *mut Self::KluNumeric, common: *mut Self::KluCommon) {
        if klu_free_numeric(numeric, common) == 0 {
            unreachable!("freeing klu numeric object failed")
        }
    }

    unsafe fn klu_z_free_numeric(
        numeric: *mut *mut Self::KluNumeric,
        common: *mut Self::KluCommon,
    ) {
        if klu_z_free_numeric(numeric, common) == 0 {
            unreachable!("freeing klu numeric object failed")
        }
    }

    fn check_status(common: &Self::KluCommon) {
        match common.status {
            -2 => unreachable!("KLU error: OUT OF MEMORY"),
            -3 => unreachable!("KLU error: INVALID"),
            -4 => unreachable!("KLU error: TOO LARGE"),
            code @ (Self::MIN..=-5 | -1) => {
                unreachable!("KLU failed with unkown errorcode {}", code)
            }
            _ => (),
        }
    }

    fn get_rcond(common: &Self::KluCommon) -> f64 {
        common.rcond
    }

    fn is_singular(common: &Self::KluCommon) -> bool {
        common.status == 1
    }

    unsafe fn klu_rcond(
        symbolic: *mut Self::KluSymbolic,
        numeric: *mut Self::KluNumeric,
        common: *mut Self::KluCommon,
    ) -> bool {
        klu_rcond(symbolic, numeric, common) != 0
    }

    unsafe fn klu_z_rcond(
        symbolic: *mut Self::KluSymbolic,
        numeric: *mut Self::KluNumeric,
        common: *mut Self::KluCommon,
    ) -> bool {
        klu_z_rcond(symbolic, numeric, common) != 0
    }
}

// Klu uses conditions to ensure its long is always 64 bit
impl KluIndex for i64 {
    type KluCommon = KluLCommon;
    type KluNumeric = KluLNumeric;
    type KluSymbolic = KluLSymbolic;

    fn from_usize(val: usize) -> Self {
        debug_assert!(val < Self::MAX as usize);
        val as Self
    }

    fn into_usize(self) -> usize {
        debug_assert!(self > 0);
        self as usize
    }

    unsafe fn klu_defaults(common: *mut Self::KluCommon) -> Self {
        klu_l_defaults(common)
    }

    unsafe fn klu_analyze(
        dim: Self,
        colum_offsets: *const Self,
        row_indices: *const Self,
        common: *mut Self::KluCommon,
    ) -> *mut Self::KluSymbolic {
        klu_l_analyze(
            dim,
            colum_offsets as *mut Self,
            row_indices as *mut Self,
            common,
        )
    }

    unsafe fn klu_free_symbolic(
        symbolic: *mut *mut Self::KluSymbolic,
        common: *mut Self::KluCommon,
    ) {
        if klu_l_free_symbolic(symbolic, common) == 0 {
            unreachable!("freeing klu numeric object failed")
        }
    }

    unsafe fn klu_factor(
        colum_offsets: *const Self,
        row_indices: *const Self,
        data: *mut f64,
        symbolic: *mut Self::KluSymbolic,
        common: *mut Self::KluCommon,
    ) -> *mut Self::KluNumeric {
        klu_l_factor(
            colum_offsets as *mut Self,
            row_indices as *mut Self,
            data,
            symbolic,
            common,
        )
    }

    unsafe fn klu_z_factor(
        colum_offsets: *const Self,
        row_indices: *const Self,
        data: *mut f64,
        symbolic: *mut Self::KluSymbolic,
        common: *mut Self::KluCommon,
    ) -> *mut Self::KluNumeric {
        klu_zl_factor(
            colum_offsets as *mut Self,
            row_indices as *mut Self,
            data,
            symbolic,
            common,
        )
    }

    unsafe fn klu_refactor(
        colum_offsets: *const Self,
        row_indices: *const Self,
        data: *mut f64,
        symbolic: *mut Self::KluSymbolic,
        numeric: *mut Self::KluNumeric,
        common: *mut Self::KluCommon,
    ) -> bool {
        klu_l_refactor(
            colum_offsets as *mut Self,
            row_indices as *mut Self,
            data,
            symbolic,
            numeric,
            common,
        ) != 0
    }

    unsafe fn klu_z_refactor(
        colum_offsets: *const Self,
        row_indices: *const Self,
        data: *mut f64,
        symbolic: *mut Self::KluSymbolic,
        numeric: *mut Self::KluNumeric,
        common: *mut Self::KluCommon,
    ) -> bool {
        klu_zl_refactor(
            colum_offsets as *mut Self,
            row_indices as *mut Self,
            data,
            symbolic,
            numeric,
            common,
        ) != 0
    }

    unsafe fn klu_solve(
        symbolic: *mut Self::KluSymbolic,
        numeric: *mut Self::KluNumeric,
        rhs_dimension: Self,
        number_rhs: Self,
        rhs_data: *mut f64,
        common: *mut Self::KluCommon,
    ) -> bool {
        klu_l_solve(
            symbolic,
            numeric,
            rhs_dimension,
            number_rhs,
            rhs_data,
            common,
        ) != 0
    }

    unsafe fn klu_tsolve(
        symbolic: *mut Self::KluSymbolic,
        numeric: *mut Self::KluNumeric,
        rhs_dimension: Self,
        number_rhs: Self,
        rhs_data: *mut f64,
        common: *mut Self::KluCommon,
    ) -> bool {
        klu_l_tsolve(
            symbolic,
            numeric,
            rhs_dimension,
            number_rhs,
            rhs_data,
            common,
        ) != 0
    }

    unsafe fn klu_z_solve(
        symbolic: *mut Self::KluSymbolic,
        numeric: *mut Self::KluNumeric,
        rhs_dimension: Self,
        number_rhs: Self,
        rhs_data: *mut f64,
        common: *mut Self::KluCommon,
    ) -> bool {
        klu_zl_solve(
            symbolic,
            numeric,
            rhs_dimension,
            number_rhs,
            rhs_data,
            common,
        ) != 0
    }

    unsafe fn klu_z_tsolve(
        symbolic: *mut Self::KluSymbolic,
        numeric: *mut Self::KluNumeric,
        rhs_dimension: Self,
        number_rhs: Self,
        rhs_data: *mut f64,
        common: *mut Self::KluCommon,
    ) -> bool {
        klu_zl_tsolve(
            symbolic,
            numeric,
            rhs_dimension,
            number_rhs,
            rhs_data,
            0,
            common,
        ) != 0
    }

    unsafe fn klu_free_numeric(numeric: *mut *mut Self::KluNumeric, common: *mut Self::KluCommon) {
        if klu_l_free_numeric(numeric, common) == 0 {
            unreachable!("freeing klu numeric object failed")
        }
    }

    unsafe fn klu_z_free_numeric(
        numeric: *mut *mut Self::KluNumeric,
        common: *mut Self::KluCommon,
    ) {
        if klu_zl_free_numeric(numeric, common) == 0 {
            unreachable!("freeing klu numeric object failed")
        }
    }

    fn check_status(common: &Self::KluCommon) {
        match common.status {
            0 => (),
            -1 => unreachable!("KLU failed with unkown errorcode -1"),
            -2 => unreachable!("KLU error: OUT OF MEMORY"),
            -3 => unreachable!("KLU error: INVALID"),
            -4 => unreachable!("KLU error: TOO LARGE"),
            code @ Self::MIN..=-5 => unreachable!("KLU failed with unkown errorcode {}", code),
            1 => unreachable!("Singular matrix!"),
            code => unreachable!("Unkown warning {code}"),
        }
    }

    fn get_rcond(common: &Self::KluCommon) -> f64 {
        common.rcond
    }

    fn is_singular(common: &Self::KluCommon) -> bool {
        common.status == 1
    }

    unsafe fn klu_rcond(
        symbolic: *mut Self::KluSymbolic,
        numeric: *mut Self::KluNumeric,
        common: *mut Self::KluCommon,
    ) -> bool {
        klu_l_rcond(symbolic, numeric, common) != 0
    }

    unsafe fn klu_z_rcond(
        symbolic: *mut Self::KluSymbolic,
        numeric: *mut Self::KluNumeric,
        common: *mut Self::KluCommon,
    ) -> bool {
        klu_zl_rcond(symbolic, numeric, common) != 0
    }
}
