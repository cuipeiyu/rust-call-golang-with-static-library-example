use std::ffi::CStr;
use std::os::raw::c_char;

#[link(name = "ping", kind = "static")]
extern "C" {
    pub fn Ping() -> *const c_char;
}

fn main() {
    println!("Hello, world!");

    let result = unsafe { Ping() };
    let c_str = unsafe { CStr::from_ptr(result) };
    let string = c_str.to_str().expect("Error to call ping function");

    println!("{}", string)
}
