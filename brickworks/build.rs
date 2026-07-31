fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "windows" {
        println!("cargo:rustc-link-lib=kernel32"); 
        println!("cargo:rustc-link-lib=psapi"); 
    }
}
