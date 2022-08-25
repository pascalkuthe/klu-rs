# klu-rs

KLU-rs are Rust binding to the KLU sparse matrix solver from [suitesparse].
KLU is a LU solve for sparse matrices that rarely/never change their sparsity pattern but often change their values.
The klu-sys crate provides raw binding to the C API while the klu crate provides a small save (opinionated) wrapper around that API.
