extern crate libc;

use libc::{c_int, size_t, c_char};
use std::ffi::CString;
use std::str;
use std::ffi::CStr;

pub const BUFFER_SIZE: size_t = 255;

#[link(name = "acbrposprinter64")]
extern {
    fn POS_Inicializar(
        eArqConfig: *const c_char,
        eChaveCrypt: *const c_char
    ) -> c_int;
    fn POS_Finalizar() -> c_int;
    fn POS_Versao(
        sVersao: *mut c_char,
        esTamanho: *mut size_t
    ) -> c_int;
}

pub fn get_version() -> Option<String> {

    let ini_file: CString = CString::new( "lib/ACBrPosPrinter.ini").unwrap();
    let chave_crypt: CString = CString::new("").unwrap();

    let ini_file_pointer: *const c_char = ini_file.as_ptr();
    let chave_crypt_pointer: *const c_char = chave_crypt.as_ptr();

    println!("POS_Inicializar exited with code: {}",
             unsafe { POS_Inicializar(ini_file_pointer, chave_crypt_pointer) as i32 });

    drop(ini_file);
    drop(chave_crypt_pointer);

    let mut versao: [c_char; 255] = [0; 255];
    let versao_pointer: *mut c_char = versao.as_mut_ptr();
    let mut versao_tamanho: size_t = 255;

    println!("POS_Versao exited with code: {}",
             unsafe { POS_Versao(versao_pointer, &mut versao_tamanho) as i32 });

    let versao_string = unsafe { CStr::from_ptr(versao_pointer) };
    let versao_string_slice = str::from_utf8(versao_string.to_bytes()).unwrap();
    let versao_string = versao_string_slice.to_string();

    println!("POS_Finalizar exited with code: {}",
             unsafe { POS_Finalizar() as i32 });

    drop(versao_pointer);
    drop(versao_tamanho);

    return Some(versao_string);
}
