extern crate libc;

use libc::{c_int, size_t, c_char, c_void};
use std::ffi::{CString, CStr};
use std::str;

pub const BUFFER_SIZE: size_t = 255;

//#[link(name = "acbrposprinter64")]
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
        panic!("Error on POS_Inicializar");
    }
}

// function to get the version of the library with panic on return codes
pub fn version() -> String {
    let mut buffer: [c_char; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut size: size_t = BUFFER_SIZE;
    let result = unsafe {
        POS_Versao(
            buffer.as_mut_ptr(),
            &mut size
        )
    };
    if result < 0 {
        panic!("Error on POS_Versao");
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
        panic!("Error on POS_Versao");
    }
}

pub fn get_version() -> Option<String> {
    init();
    let versao_string = version();
    finalize();
    Some(versao_string)
}


pub fn pos_version(mut cx: FunctionContext) -> JsResult<JsString> {
    let versao_string = get_version().unwrap();
    return Ok(cx.string(versao_string));
}


use neon::prelude::*;

fn get_num_cpus(mut cx: FunctionContext) -> JsResult<JsNumber> {
    Ok(cx.number(num_cpus::get() as f64))
}

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("num_cpus", get_num_cpus)?;
    cx.export_function("version", pos_version)?;
    Ok(())
}

