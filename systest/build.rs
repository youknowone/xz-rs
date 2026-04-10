use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn target_deps_dir() -> PathBuf {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir.parent().unwrap();
    let target_root = env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| workspace_root.join("target"));
    let profile = env::var("PROFILE").unwrap();
    let target = env::var("TARGET").unwrap();

    let triple_root = target_root.join(&target);
    let profile_root = if triple_root.exists() {
        triple_root.join(&profile)
    } else {
        target_root.join(&profile)
    };
    profile_root.join("deps")
}

fn latest_rlib(deps_dir: &Path, crate_name: &str) -> PathBuf {
    let prefix = format!("lib{crate_name}-");
    fs::read_dir(deps_dir)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            let file_name = path.file_name()?.to_str()?;
            (file_name.starts_with(&prefix) && path.extension().is_some_and(|ext| ext == "rlib"))
                .then_some(path)
        })
        .max_by_key(|path| fs::metadata(path).and_then(|m| m.modified()).ok())
        .unwrap_or_else(|| {
            panic!(
                "missing compiled dependency for {crate_name} in {}",
                deps_dir.display()
            )
        })
}

fn write_rustc_wrapper(
    out_dir: &Path,
    deps_dir: &Path,
    bare_externs: &[&str],
    path_externs: &[(&str, PathBuf)],
) -> PathBuf {
    let real_rustc = env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string());
    let wrapper = if cfg!(windows) {
        out_dir.join("ctest-rustc.bat")
    } else {
        out_dir.join("ctest-rustc.sh")
    };

    let mut args = format!("-L dependency=\"{}\"", deps_dir.display());
    for name in bare_externs {
        args.push_str(&format!(" --extern {name}"));
    }
    for (name, path) in path_externs {
        args.push_str(&format!(" --extern {name}=\"{}\"", path.display()));
    }

    let script = if cfg!(windows) {
        format!("@echo off\r\n\"{real_rustc}\" {args} %*\r\n")
    } else {
        format!("#!/bin/sh\nexec \"{real_rustc}\" {args} \"$@\"\n")
    };

    fs::write(&wrapper, script).unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&wrapper).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&wrapper, perms).unwrap();
    }

    wrapper
}

fn main() {
    let use_c_sys = env::var_os("CARGO_FEATURE_LIBLZMA_SYS").is_some();
    let use_rs_sys = env::var_os("CARGO_FEATURE_XZ_SYS").is_some();
    let use_parallel = env::var_os("CARGO_FEATURE_PARALLEL").is_some();
    match (use_c_sys, use_rs_sys) {
        (true, false) | (false, true) => {}
        _ => panic!("Enable exactly one of features: liblzma-sys or xz-sys"),
    }

    let mut cfg = ctest::TestGenerator::new();
    let use_bindgen = env::var_os("CARGO_FEATURE_BINDGEN").is_some();
    if use_bindgen {
        cfg.cfg("feature", Some("bindgen"));
    }
    if use_c_sys {
        if let Ok(out) = env::var("DEP_LZMA_INCLUDE") {
            cfg.include(&out);
        } else {
            // pkg-config-backed liblzma-sys builds can return early without exporting
            // cargo:include metadata. Fall back to the vendored upstream API headers so
            // systest can still compile its generated probe.
            cfg.include("../liblzma-sys/xz/src/liblzma/api");
        }
    } else {
        // Reuse vendored upstream headers to verify C header compatibility.
        cfg.include("../liblzma-sys/xz/src/liblzma/api");
    }

    cfg.header("lzma.h");
    cfg.rename_struct_ty(|ty| Some(ty.to_string()));
    cfg.rename_union_ty(|ty| Some(ty.to_string()));
    cfg.rename_struct_field(|s, field| {
        if s.ident() == "lzma_options_delta" && field.ident() == "type_" {
            Some("type".to_string())
        } else {
            None
        }
    });
    cfg.define("LZMA_API_STATIC", None);
    cfg.skip_struct(move |s| {
        use_bindgen && (s.ident().ends_with("_s") || s.ident().contains("__bindgen_ty_"))
    });
    cfg.skip_union(move |u| use_bindgen && u.ident().contains("__bindgen_ty_"));
    cfg.skip_struct_field(move |s, field| {
        use_bindgen
            && s.ident() == "lzma_index_iter"
            && matches!(field.ident(), "stream" | "block" | "internal")
    });
    cfg.skip_struct_field_type(move |s, field| {
        use_bindgen
            && s.ident() == "lzma_index_iter"
            && matches!(field.ident(), "stream" | "block" | "internal")
    });
    cfg.skip_fn(move |f| {
        use_bindgen
            && !use_parallel
            && matches!(
                f.ident(),
                "lzma_stream_decoder_mt"
                    | "lzma_stream_encoder_mt"
                    | "lzma_stream_encoder_mt_memusage"
            )
    });
    cfg.skip_alias(move |n| {
        matches!(n.ident(), "__enum_ty" | "c_enum" | "lzma_reserved_enum")
            || (use_bindgen
                && matches!(
                    n.ident(),
                    "lzma_internal" | "lzma_index" | "lzma_index_hash"
                ))
    });
    cfg.skip_signededness(move |ty| {
        use_bindgen && matches!(ty, "lzma_delta_type" | "lzma_index_iter_mode")
    });
    if use_bindgen {
        cfg.skip_const(|c| {
            c.ident().starts_with("lzma_")
                || matches!(
                    c.ident(),
                    "LZMA_H_INTERNAL" | "LZMA_VERSION_COMMIT" | "LZMA_VERSION_STABILITY_STRING"
                )
        });
    }

    let rust_api = if use_c_sys {
        "../liblzma-sys/src/lib.rs"
    } else {
        "../xz-sys/src/lib.rs"
    };

    let deps_dir = target_deps_dir();
    let bare_externs = vec!["libc"];
    let mut path_externs = Vec::new();
    if use_rs_sys {
        path_externs.push(("xz", latest_rlib(&deps_dir, "xz")));
    }
    let rustc_wrapper = write_rustc_wrapper(
        &PathBuf::from(env::var_os("OUT_DIR").unwrap()),
        &deps_dir,
        &bare_externs,
        &path_externs,
    );
    env::set_var("RUSTC", &rustc_wrapper);

    ctest::generate_test(&mut cfg, rust_api, "all.rs").unwrap();
}
