use std::{env, path::PathBuf};

fn main() {
    let g711_c = {
        let mut path: PathBuf = env::var("CARGO_MANIFEST_DIR").unwrap().into();
        path.push("reference");
        path.push("g711.c");
        path
    };

    cc::Build::new().file(&g711_c).compile("g711");
}
