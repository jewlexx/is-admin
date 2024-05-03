#[cfg(feature = "std")]
fn main() {
    use quork::prelude::is_root;
    let root = is_root().unwrap();

    println!("Is the the process running as admin? {}", root);
}

#[cfg(not(feature = "std"))]
fn main() {
    panic!("This example requires the standard crate")
}
