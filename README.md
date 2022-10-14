# rust-call-golang-with-static-library-example

## Some caveats

- **0x00** Config .cargo/config
```toml
# add
[build]
rustflags = ["-C", "link-args=-framework CoreFoundation -framework Security"]

```

- **0x01** build.rs

```shell
$ cat build.rs
use std::env;

fn main() {
	let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
	println!("cargo:rustc-link-search={}/libping", dir);
	// or
	println!("cargo:rustc-link-search=native=./libping");

	// library name, just name, no 'lib'
	println!("cargo:rustc-link-lib=static=ping");
}

```

- **0x02** Golang build
```shell
# must enable cgo to build
set CGO_ENABLED=1

```
