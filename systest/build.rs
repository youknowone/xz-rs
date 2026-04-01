use std::env;

fn main() {
    let use_c_sys = env::var_os("CARGO_FEATURE_C_SYS").is_some();
    let use_rs_sys = env::var_os("CARGO_FEATURE_RS_SYS").is_some();
    match (use_c_sys, use_rs_sys) {
        (true, false) | (false, true) => {}
        _ => panic!("Enable exactly one of features: c-sys or rs-sys"),
    }

    let mut cfg = ctest2::TestGenerator::new();
    if use_c_sys {
        if let Ok(out) = env::var("DEP_LZMA_INCLUDE") {
            cfg.include(&out);
        }
    } else {
        // Reuse vendored upstream headers to verify C header compatibility.
        cfg.include("../liblzma-sys/xz/src/liblzma/api");
    }

    cfg.header("lzma.h");
    cfg.type_name(|n, _s, _| n.to_string());
    cfg.define("LZMA_API_STATIC", None);
    cfg.skip_type(|n| n == "__enum_ty");

    let rust_api = if use_c_sys {
        "../liblzma-sys/src/lib.rs"
    } else {
        "../liblzma-rs-sys/src/lib.rs"
    };
    cfg.generate(rust_api, "all.rs");
}
