use std::path::PathBuf;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    // OUT_DIR = target/<triple>/<profile>/build/<crate>-<hash>/out
    let deps_dir = PathBuf::from(&out_dir)
        .ancestors()
        .nth(3)
        .expect("unexpected OUT_DIR layout")
        .join("deps");

    println!("cargo:rustc-link-search=native={}", deps_dir.display());
    println!("cargo:rustc-link-lib=dylib=brickworks");
}
