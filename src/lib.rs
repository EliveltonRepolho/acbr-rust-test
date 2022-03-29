extern crate libc;

use node_bindgen::derive::node_bindgen;
use libc::{c_int, size_t, c_char, c_void};
use std::ffi::{CString, CStr};
use std::str;

pub const BUFFER_SIZE: size_t = 255;

#[link(name = "acbrposprinter64")]
extern {
    fn POS_Inicializar(
        eArqConfig: *const c_char,
        eChaveCrypt: *const c_char
    ) -> c_int;
    fn POS_Finalizar(void: *const c_void) -> c_int;
    fn POS_Versao(
        sVersao: *mut c_char,
        esTamanho: *mut size_t
    ) -> c_int;
}


pub fn init()  {
    #[allow(temporary_cstring_as_ptr)]
    let result = unsafe {
        POS_Inicializar(
            CString::new("lib/ACBrPosPrinter.ini").unwrap().as_ptr(),
            CString::new("").unwrap().as_ptr()
        ) as i32
    };

    if result < 0 {
        panic!("Error on POS_Inicializar: {}", result);
    }
}

// function to get the version of the library with panic on return codes
pub fn pos_version() -> String {
    let mut buffer: [c_char; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut size: size_t = BUFFER_SIZE;
    let result = unsafe {
        POS_Versao(
            buffer.as_mut_ptr(),
            &mut size
        )
    };
    if result < 0 {
        panic!("Error on POS_Versao: {}", result);
    }
    let c_str = unsafe { CStr::from_ptr(buffer.as_ptr()) };
    str::from_utf8(c_str.to_bytes()).unwrap().to_string()
}


// function to finalize the library
pub fn finalize() {
    let result= unsafe {
        POS_Finalizar(std::ptr::null())
    };

    if result < 0 {
        panic!("Error on POS_Versao: {}", result);
    }
}


#[node_bindgen(name="get_version")]
pub fn get_version() -> String {
    init();
    let versao_string = pos_version();
    finalize();
    versao_string
}

#[node_bindgen(name="num_cpus")]
fn num_cpus() -> i32 {
    num_cpus::get() as i32
}

#[node_bindgen(name="x_hello")]
fn x_hello() -> String {
    "hello node-bindgen".to_string()
}

#[node_bindgen(name="hello")]
fn hello() -> String {
    "hello node-bindgen".to_string()
}
