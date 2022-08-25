use std::path::Path;

fn main() {
    if std::env::var_os("CARGO_FEATURE_DYNAMIC").is_some() {
        println!("cargo:rustc-link-lib=klu");
    } else {
        let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("vendor");
        build_suitesparse_config(&src_dir);
        for long in [true, false] {
            build_amd(&src_dir, long);
            build_colamd(&src_dir, long);
            build_btf(&src_dir, long);
            build_amd(&src_dir, long);
            build_klu_common(&src_dir, long);
            for complex in [true, false] {
                build_klu(&src_dir, long, complex);
            }
        }
    }
}

fn build_suitesparse_config(src_dir: &Path) {
    cc::Build::new()
        .file(
            src_dir
                .join("SuiteSparse_config")
                .join("SuiteSparse_config.c"),
        )
        .compile("suitesparseconfig");
}

fn setup_suitesparse_builder(long: bool, src_dir: &Path) -> cc::Build {
    let mut builder = cc::Build::new();

    if long {
        builder.define("DLONG", None);
    }

    builder.include(src_dir.join("SuiteSparse_config"));
    builder
}

fn build_amd(src_dir: &Path, long: bool) {
    let mut builder = setup_suitesparse_builder(long, src_dir);
    let dir = src_dir.join("AMD");
    let amd_src_dir = dir.join("Source");

    builder.include(dir.join("Include"));

    let amd_objects = [
        "amd_1",
        "amd_2",
        "amd_aat",
        "amd_control",
        "amd_defaults",
        "amd_dump",
        "amd_global",
        "amd_info",
        "amd_order",
        "amd_post_tree",
        "amd_postorder",
        "amd_preprocess",
        "amd_valid",
    ];

    for obj in amd_objects.iter() {
        builder.file(&amd_src_dir.join(format!("{obj}.c")));
    }

    builder.compile(if long { "amdl" } else { "amd" });
}

fn build_colamd(src_dir: &Path, long: bool) {
    let mut builder = setup_suitesparse_builder(long, src_dir);

    let dir = src_dir.join("COLAMD");
    let src = dir.join("Source").join("colamd.c");
    builder.include(dir.join("Include")).file(src);

    builder.compile(if long { "colamdl" } else { "colamd" });
}

fn build_btf(src_dir: &Path, long: bool) {
    let mut builder = setup_suitesparse_builder(long, src_dir);

    let dir = src_dir.join("BTF");
    let btf_src_dir = dir.join("Source");
    builder.include(dir.join("Include"));

    let btf_objects = ["btf_maxtrans", "btf_order", "btf_strongcomp"];
    for obj in btf_objects.iter() {
        builder.file(&btf_src_dir.join(format!("{}.c", obj)));
    }

    builder.compile(if long { "btfl" } else { "btf" });
}

fn build_klu_common(src_dir: &Path, long: bool) {
    let klu_src = src_dir.join("KLU").join("Source");

    let mut builder = setup_suitesparse_builder(long, src_dir);
    let objects = [
        "_analyze",
        "_analyze_given",
        "_defaults",
        "_memory",
        "_free_symbolic",
    ];

    for obj in &objects {
        builder.file(klu_src.join(format!("klu{}.c", obj)));
    }

    builder
        .include(src_dir.join("KLU").join("Include"))
        .include(src_dir.join("AMD").join("Include"))
        .include(src_dir.join("COLAMD").join("Include"))
        .include(src_dir.join("BTF").join("Include"))
        .include(src_dir.join("SuiteSparse_config"))
        .compile(if long { "klul_common" } else { "klu_common" });
}

fn build_klu(src_dir: &Path, long: bool, complex: bool) {
    let klu_src = src_dir.join("KLU").join("Source");

    let mut builder = setup_suitesparse_builder(long, src_dir);
    let mut name = "klu".to_string();

    if long {
        builder.define("DLONG", None);
        name.push('l');
    }

    if complex {
        builder.define("COMPLEX", None);
        name.push('z');
    }

    let objects = [
        "",
        "_diagnostics",
        "_dump",
        "_factor",
        "_free_numeric",
        "_kernel",
        "_refactor",
        "_scale",
        "_solve",
        "_sort",
        "_tsolve",
    ];

    for obj in &objects {
        builder.file(klu_src.join(format!("klu{}.c", obj)));
    }

    builder
        .include(src_dir.join("KLU").join("Include"))
        .include(src_dir.join("AMD").join("Include"))
        .include(src_dir.join("COLAMD").join("Include"))
        .include(src_dir.join("BTF").join("Include"))
        .include(src_dir.join("SuiteSparse_config"))
        .compile(&name);
}
