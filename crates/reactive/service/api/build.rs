#[rustversion::nightly]
fn main() {
    println!("cargo:rustc-cfg=unboxed_closures");
    println!("cargo:rustc-cfg=fn_traits");
}

#[rustversion::not(nightly)]
fn main() {}
