//! Conditionally compile the g191 implementations if the `g191` or `g191-sys` features are enabled.

#[cfg(any(feature = "g191", feature = "g191-sys"))]
fn main() {
    use std::{env, path::PathBuf};

    let reference_dir = {
        let mut path: PathBuf = env::var("CARGO_MANIFEST_DIR").unwrap().into();
        path.push("reference");
        path
    };
    let g711_c = reference_dir.join("g711.c");
    let g711_h = reference_dir.join("g711.h");

    println!("cargo::rerun-if-changed={}", g711_c.display());
    println!("cargo::rerun-if-changed={}", g711_h.display());

    cc::Build::new().file(&g711_c).compile("g711");
}

#[cfg(not(any(feature = "g191", feature = "g191-sys")))]
fn main() {}
