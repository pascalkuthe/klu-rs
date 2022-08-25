use std::os::raw::c_ulong;
#[allow(non_camel_case_types)]
pub type size_t = c_ulong;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KluSymbolic {
    _data: [f64; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KluLSymbolic {
    _data: [f64; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KluNumeric {
    _data: [f64; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KluLNumeric {
    _data: [f64; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KluLCommon {
    pub tol: f64,
    pub memgrow: f64,
    pub initmem_amd: f64,
    pub initmem: f64,
    pub maxwork: f64,
    pub btf: i64,
    pub ordering: i64,
    pub scale: i64,
    pub user_order: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: i64,
            arg2: *mut i64,
            arg3: *mut i64,
            arg4: *mut i64,
            arg5: *mut KluLCommon,
        ) -> i64,
    >,
    pub user_data: *mut ::std::os::raw::c_void,
    pub halt_if_singular: i64,
    pub status: i64,
    pub nrealloc: i64,
    pub structural_rank: i64,
    pub numerical_rank: i64,
    pub singular_col: i64,
    pub noffdiag: i64,
    pub flops: f64,
    pub rcond: f64,
    pub condest: f64,
    pub rgrowth: f64,
    pub work: f64,
    pub memusage: size_t,
    pub mempeak: size_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KluCommon {
    pub tol: f64,
    pub memgrow: f64,
    pub initmem_amd: f64,
    pub initmem: f64,
    pub maxwork: f64,
    pub btf: i32,
    pub ordering: i32,
    pub scale: i32,
    pub user_order: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: i32,
            arg2: *mut i32,
            arg3: *mut i32,
            arg4: *mut i32,
            arg5: *mut KluCommon,
        ) -> i32,
    >,
    pub user_data: *mut ::std::os::raw::c_void,
    pub halt_if_singular: i32,
    pub status: i32,
    pub nrealloc: i32,
    pub structural_rank: i32,
    pub numerical_rank: i32,
    pub singular_col: i32,
    pub noffdiag: i32,
    pub flops: f64,
    pub rcond: f64,
    pub condest: f64,
    pub rgrowth: f64,
    pub work: f64,
    pub memusage: size_t,
    pub mempeak: size_t,
}

extern "C" {
    pub fn klu_defaults(Common: *mut KluCommon) -> i32;
    pub fn klu_l_defaults(Common: *mut KluLCommon) -> i64;

    pub fn klu_analyze(
        n: i32,
        Ap: *const i32,
        Ai: *const i32,
        Common: *mut KluCommon,
    ) -> *mut KluSymbolic;

    pub fn klu_l_analyze(
        arg1: i64,
        arg2: *mut i64,
        arg3: *mut i64,
        Common: *mut KluLCommon,
    ) -> *mut KluLSymbolic;

    pub fn klu_analyze_given(
        n: i32,
        Ap: *const i32,
        Ai: *const i32,
        P: *mut i32,
        Q: *mut i32,
        Common: *mut KluCommon,
    ) -> *mut KluSymbolic;

    pub fn klu_l_analyze_given(
        arg1: i64,
        arg2: *mut i64,
        arg3: *mut i64,
        arg4: *mut i64,
        arg5: *mut i64,
        arg6: *mut KluLCommon,
    ) -> *mut KluLSymbolic;

    pub fn klu_factor(
        Ap: *const i32,
        Ai: *const i32,
        Ax: *mut f64,
        Symbolic: *mut KluSymbolic,
        Common: *mut KluCommon,
    ) -> *mut KluNumeric;

    pub fn klu_z_factor(
        Ap: *const i32,
        Ai: *const i32,
        Ax: *mut f64,
        Symbolic: *mut KluSymbolic,
        Common: *mut KluCommon,
    ) -> *mut KluNumeric;

    pub fn klu_l_factor(
        arg1: *mut i64,
        arg2: *mut i64,
        arg3: *mut f64,
        arg4: *mut KluLSymbolic,
        arg5: *mut KluLCommon,
    ) -> *mut KluLNumeric;

    pub fn klu_zl_factor(
        arg1: *mut i64,
        arg2: *mut i64,
        arg3: *mut f64,
        arg4: *mut KluLSymbolic,
        arg5: *mut KluLCommon,
    ) -> *mut KluLNumeric;

    pub fn klu_solve(
        Symbolic: *mut KluSymbolic,
        Numeric: *mut KluNumeric,
        ldim: i32,
        nrhs: i32,
        B: *mut f64,
        Common: *mut KluCommon,
    ) -> i32;

    pub fn klu_z_solve(
        Symbolic: *mut KluSymbolic,
        Numeric: *mut KluNumeric,
        ldim: i32,
        nrhs: i32,
        B: *mut f64,
        Common: *mut KluCommon,
    ) -> i32;

    pub fn klu_l_solve(
        arg1: *mut KluLSymbolic,
        arg2: *mut KluLNumeric,
        arg3: i64,
        arg4: i64,
        arg5: *mut f64,
        arg6: *mut KluLCommon,
    ) -> i64;

    pub fn klu_zl_solve(
        arg1: *mut KluLSymbolic,
        arg2: *mut KluLNumeric,
        arg3: i64,
        arg4: i64,
        arg5: *mut f64,
        arg6: *mut KluLCommon,
    ) -> i64;

    pub fn klu_tsolve(
        Symbolic: *mut KluSymbolic,
        Numeric: *mut KluNumeric,
        ldim: i32,
        nrhs: i32,
        B: *mut f64,
        Common: *mut KluCommon,
    ) -> i32;

    pub fn klu_z_tsolve(
        Symbolic: *mut KluSymbolic,
        Numeric: *mut KluNumeric,
        ldim: i32,
        nrhs: i32,
        B: *mut f64,
        conj_solve: i32,
        Common: *mut KluCommon,
    ) -> i32;

    pub fn klu_l_tsolve(
        arg1: *mut KluLSymbolic,
        arg2: *mut KluLNumeric,
        arg3: i64,
        arg4: i64,
        arg5: *mut f64,
        arg6: *mut KluLCommon,
    ) -> i64;

    pub fn klu_zl_tsolve(
        arg1: *mut KluLSymbolic,
        arg2: *mut KluLNumeric,
        arg3: i64,
        arg4: i64,
        arg5: *mut f64,
        arg6: i64,
        arg7: *mut KluLCommon,
    ) -> i64;

    pub fn klu_refactor(
        Ap: *const i32,
        Ai: *const i32,
        Ax: *mut f64,
        Symbolic: *mut KluSymbolic,
        Numeric: *mut KluNumeric,
        Common: *mut KluCommon,
    ) -> i32;

    pub fn klu_z_refactor(
        Ap: *const i32,
        Ai: *const i32,
        Ax: *mut f64,
        Symbolic: *mut KluSymbolic,
        Numeric: *mut KluNumeric,
        Common: *mut KluCommon,
    ) -> i32;

    pub fn klu_l_refactor(
        arg1: *mut i64,
        arg2: *mut i64,
        arg3: *mut f64,
        arg4: *mut KluLSymbolic,
        arg5: *mut KluLNumeric,
        arg6: *mut KluLCommon,
    ) -> i64;

    pub fn klu_zl_refactor(
        arg1: *mut i64,
        arg2: *mut i64,
        arg3: *mut f64,
        arg4: *mut KluLSymbolic,
        arg5: *mut KluLNumeric,
        arg6: *mut KluLCommon,
    ) -> i64;

    pub fn klu_free_symbolic(Symbolic: *mut *mut KluSymbolic, Common: *mut KluCommon) -> i32;

    pub fn klu_l_free_symbolic(arg1: *mut *mut KluLSymbolic, arg2: *mut KluLCommon) -> i64;

    pub fn klu_free_numeric(Numeric: *mut *mut KluNumeric, Common: *mut KluCommon) -> i32;

    pub fn klu_z_free_numeric(Numeric: *mut *mut KluNumeric, Common: *mut KluCommon) -> i32;

    pub fn klu_l_free_numeric(arg1: *mut *mut KluLNumeric, arg2: *mut KluLCommon) -> i64;

    pub fn klu_zl_free_numeric(arg1: *mut *mut KluLNumeric, arg2: *mut KluLCommon) -> i64;

    pub fn klu_sort(
        Symbolic: *mut KluSymbolic,
        Numeric: *mut KluNumeric,
        Common: *mut KluCommon,
    ) -> i32;

    pub fn klu_z_sort(
        Symbolic: *mut KluSymbolic,
        Numeric: *mut KluNumeric,
        Common: *mut KluCommon,
    ) -> i32;

    pub fn klu_l_sort(
        arg1: *mut KluLSymbolic,
        arg2: *mut KluLNumeric,
        arg3: *mut KluLCommon,
    ) -> i64;

    pub fn klu_zl_sort(
        arg1: *mut KluLSymbolic,
        arg2: *mut KluLNumeric,
        arg3: *mut KluLCommon,
    ) -> i64;

    pub fn klu_flops(
        Symbolic: *mut KluSymbolic,
        Numeric: *mut KluNumeric,
        Common: *mut KluCommon,
    ) -> i32;

    pub fn klu_z_flops(
        Symbolic: *mut KluSymbolic,
        Numeric: *mut KluNumeric,
        Common: *mut KluCommon,
    ) -> i32;

    pub fn klu_l_flops(
        arg1: *mut KluLSymbolic,
        arg2: *mut KluLNumeric,
        arg3: *mut KluLCommon,
    ) -> i64;

    pub fn klu_zl_flops(
        arg1: *mut KluLSymbolic,
        arg2: *mut KluLNumeric,
        arg3: *mut KluLCommon,
    ) -> i64;

    pub fn klu_rgrowth(
        Ap: *const i32,
        Ai: *const i32,
        Ax: *mut f64,
        Symbolic: *mut KluSymbolic,
        Numeric: *mut KluNumeric,
        Common: *mut KluCommon,
    ) -> i32;

    pub fn klu_z_rgrowth(
        Ap: *const i32,
        Ai: *const i32,
        Ax: *mut f64,
        Symbolic: *mut KluSymbolic,
        Numeric: *mut KluNumeric,
        Common: *mut KluCommon,
    ) -> i32;

    pub fn klu_l_rgrowth(
        arg1: *mut i64,
        arg2: *mut i64,
        arg3: *mut f64,
        arg4: *mut KluLSymbolic,
        arg5: *mut KluLNumeric,
        arg6: *mut KluLCommon,
    ) -> i64;

    pub fn klu_zl_rgrowth(
        arg1: *mut i64,
        arg2: *mut i64,
        arg3: *mut f64,
        arg4: *mut KluLSymbolic,
        arg5: *mut KluLNumeric,
        arg6: *mut KluLCommon,
    ) -> i64;

    pub fn klu_condest(
        Ap: *const i32,
        Ax: *mut f64,
        Symbolic: *mut KluSymbolic,
        Numeric: *mut KluNumeric,
        Common: *mut KluCommon,
    ) -> i32;

    pub fn klu_z_condest(
        Ap: *const i32,
        Ax: *mut f64,
        Symbolic: *mut KluSymbolic,
        Numeric: *mut KluNumeric,
        Common: *mut KluCommon,
    ) -> i32;

    pub fn klu_l_condest(
        arg1: *mut i64,
        arg2: *mut f64,
        arg3: *mut KluLSymbolic,
        arg4: *mut KluLNumeric,
        arg5: *mut KluLCommon,
    ) -> i64;

    pub fn klu_zl_condest(
        arg1: *mut i64,
        arg2: *mut f64,
        arg3: *mut KluLSymbolic,
        arg4: *mut KluLNumeric,
        arg5: *mut KluLCommon,
    ) -> i64;

    pub fn klu_rcond(
        Symbolic: *mut KluSymbolic,
        Numeric: *mut KluNumeric,
        Common: *mut KluCommon,
    ) -> i32;

    pub fn klu_z_rcond(
        Symbolic: *mut KluSymbolic,
        Numeric: *mut KluNumeric,
        Common: *mut KluCommon,
    ) -> i32;

    pub fn klu_l_rcond(
        arg1: *mut KluLSymbolic,
        arg2: *mut KluLNumeric,
        arg3: *mut KluLCommon,
    ) -> i64;

    pub fn klu_zl_rcond(
        arg1: *mut KluLSymbolic,
        arg2: *mut KluLNumeric,
        arg3: *mut KluLCommon,
    ) -> i64;

    pub fn klu_scale(
        scale: i32,
        n: i32,
        Ap: *const i32,
        Ai: *const i32,
        Ax: *mut f64,
        Rs: *mut f64,
        W: *mut i32,
        Common: *mut KluCommon,
    ) -> i32;

    pub fn klu_z_scale(
        scale: i32,
        n: i32,
        Ap: *const i32,
        Ai: *const i32,
        Ax: *mut f64,
        Rs: *mut f64,
        W: *mut i32,
        Common: *mut KluCommon,
    ) -> i32;

    pub fn klu_l_scale(
        arg1: i64,
        arg2: i64,
        arg3: *mut i64,
        arg4: *mut i64,
        arg5: *mut f64,
        arg6: *mut f64,
        arg7: *mut i64,
        arg8: *mut KluLCommon,
    ) -> i64;

    pub fn klu_zl_scale(
        arg1: i64,
        arg2: i64,
        arg3: *mut i64,
        arg4: *mut i64,
        arg5: *mut f64,
        arg6: *mut f64,
        arg7: *mut i64,
        arg8: *mut KluLCommon,
    ) -> i64;

    pub fn klu_extract(
        Numeric: *mut KluNumeric,
        Symbolic: *mut KluSymbolic,
        Lp: *mut i32,
        Li: *mut i32,
        Lx: *mut f64,
        Up: *mut i32,
        Ui: *mut i32,
        Ux: *mut f64,
        Fp: *mut i32,
        Fi: *mut i32,
        Fx: *mut f64,
        P: *mut i32,
        Q: *mut i32,
        Rs: *mut f64,
        R: *mut i32,
        Common: *mut KluCommon,
    ) -> i32;

    pub fn klu_z_extract(
        Numeric: *mut KluNumeric,
        Symbolic: *mut KluSymbolic,
        Lp: *mut i32,
        Li: *mut i32,
        Lx: *mut f64,
        Lz: *mut f64,
        Up: *mut i32,
        Ui: *mut i32,
        Ux: *mut f64,
        Uz: *mut f64,
        Fp: *mut i32,
        Fi: *mut i32,
        Fx: *mut f64,
        Fz: *mut f64,
        P: *mut i32,
        Q: *mut i32,
        Rs: *mut f64,
        R: *mut i32,
        Common: *mut KluCommon,
    ) -> i32;

    pub fn klu_l_extract(
        arg1: *mut KluLNumeric,
        arg2: *mut KluLSymbolic,
        arg3: *mut i64,
        arg4: *mut i64,
        arg5: *mut f64,
        arg6: *mut i64,
        arg7: *mut i64,
        arg8: *mut f64,
        arg9: *mut i64,
        arg10: *mut i64,
        arg11: *mut f64,
        arg12: *mut i64,
        arg13: *mut i64,
        arg14: *mut f64,
        arg15: *mut i64,
        arg16: *mut KluLCommon,
    ) -> i64;

    pub fn klu_zl_extract(
        arg1: *mut KluLNumeric,
        arg2: *mut KluLSymbolic,
        arg3: *mut i64,
        arg4: *mut i64,
        arg5: *mut f64,
        arg6: *mut f64,
        arg7: *mut i64,
        arg8: *mut i64,
        arg9: *mut f64,
        arg10: *mut f64,
        arg11: *mut i64,
        arg12: *mut i64,
        arg13: *mut f64,
        arg14: *mut f64,
        arg15: *mut i64,
        arg16: *mut i64,
        arg17: *mut f64,
        arg18: *mut i64,
        arg19: *mut KluLCommon,
    ) -> i64;
}
