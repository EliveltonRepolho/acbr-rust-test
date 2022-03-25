use libc::{c_int, size_t, c_char};
use std::ffi::{CString,CStr};
use std::{ptr, str};
use acbr_rust::{get_version};


fn main() {
    let version = get_version();
    println!("{}", version);

}