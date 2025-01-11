fn main() {
    use std::{env, path::PathBuf};

    let reference_dir = {
        let mut path: PathBuf = env::var("CARGO_MANIFEST_DIR").unwrap().into();
        path.push("reference");
        path
    };
    let g711_c = reference_dir.join("g711.c");
    let g711_h = reference_dir.join("g711.h");

    let g711_h_rs = {
        let mut path: PathBuf = env::var("OUT_DIR").unwrap().into();
        path.push("g711.h.rs");
        println!("cargo::rustc-env=G711_H_RS={}", path.display());
        path
    };

    bindgen::builder()
        .header(g711_h.display().to_string())
        .use_core()
        .allowlist_function("ulaw.*")
        .generate()
        .unwrap()
        .write_to_file(&g711_h_rs)
        .unwrap();

    cc::Build::new().file(&g711_c).compile("g711");
}
