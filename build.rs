use std::env;

fn main() {
	let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
	println!(r"cargo:rustc-link-search={}/libping", dir);
	// or
	println!("cargo:rustc-link-search=native=./libping");

	// library name, just name, no 'lib'
	println!("cargo:rustc-link-lib=static=ping");
}
