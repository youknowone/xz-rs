use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn target_deps_dir() -> PathBuf {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir.parent().unwrap();
    let target_root = env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .map(|path| {
            if path.is_relative() {
                workspace_root.join(path)
            } else {
                path
            }
        })
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
    let use_rs_sys = env::var_os("CARGO_FEATURE_XZ_SYS").is_some();
    let use_parallel = env::var_os("CARGO_FEATURE_PARALLEL").is_some();
    if !use_rs_sys {
        panic!("Enable feature: xz-sys");
    }

    let mut cfg = ctest::TestGenerator::new();
    // Reuse vendored upstream headers to verify C header compatibility.
    cfg.include("../vendor/xz/src/liblzma/api");

    cfg.header("lzma.h");
    cfg.rename_struct_ty(|ty| Some(ty.to_string()));
    cfg.rename_union_ty(|ty| Some(ty.to_string()));
    cfg.define("LZMA_API_STATIC", None);
    cfg.skip_struct(move |s| matches!(s.ident(), "StaticAllocator"));
    cfg.skip_static(move |s| matches!(s.ident(), "C_ALLOCATOR"));
    cfg.skip_fn(move |f| {
        (!use_parallel
            && matches!(
                f.ident(),
                "lzma_stream_decoder_mt"
                    | "lzma_stream_encoder_mt"
                    | "lzma_stream_encoder_mt_memusage"
            ))
            || matches!(f.ident(), "malloc" | "calloc" | "free")
    });
    cfg.skip_alias(move |n| matches!(n.ident(), "__enum_ty" | "c_enum" | "lzma_reserved_enum"));

    let rust_api = "../xz-sys/src/lib.rs";

    let deps_dir = target_deps_dir();
    let bare_externs = vec!["libc"];
    let path_externs = vec![("xz_core", latest_rlib(&deps_dir, "xz_core"))];
    let rustc_wrapper = write_rustc_wrapper(
        &PathBuf::from(env::var_os("OUT_DIR").unwrap()),
        &deps_dir,
        &bare_externs,
        &path_externs,
    );
    env::set_var("RUSTC", &rustc_wrapper);
    ctest::generate_test(&mut cfg, rust_api, "all.rs").unwrap();
}
