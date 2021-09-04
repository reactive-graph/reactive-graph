fn main() {
    // Pass the version number of rustc through as an environment variable
    let version = rustc_version::version().unwrap();
    println!("cargo:rustc-env=RUSTC_VERSION={}", version);
}
