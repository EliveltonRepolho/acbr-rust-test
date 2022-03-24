extern crate libloading;

use std::ffi::{CString};

use libloading::{Library, Symbol};

use std::os::raw::c_char;

type Inicializar = extern "system" fn(eArqConfig: *const c_char, eChaveCrypt: *const c_char) -> i32;
type Finalizar = extern "system" fn() -> i32;
type Versao = extern "C" fn(sVersao: *mut c_char, esTamanho: *const i32) -> i32;

fn main() {

    let ini_file = CString::new( "abcr_lib/acbrlib.ini").unwrap();
    let ini_file_pointer = ini_file.as_ptr();

    let chave_crypt = CString::new("").unwrap();
    let chave_crypt_pointer = chave_crypt.as_ptr();

    let library_path = "abcr_lib/libacbrposprinter64.so";
    println!("Loading acbr from {}", library_path);

    let lib = unsafe { Library::new(library_path).unwrap() };

    let inicializar: Symbol<Inicializar> = unsafe { lib.get(CString::new("POS_Inicializar").unwrap().as_bytes()).unwrap() };
    println!("inicializar = {}", inicializar(ini_file_pointer, chave_crypt_pointer));

    let buffer = CString::default();
    let buffer_raw = buffer.into_raw();

    let mut buffer_tamanho: i32 = 0;
    let buffer_tamanho_reference = &mut buffer_tamanho;

    let versao: Symbol<Versao> = unsafe { lib.get(CString::new("POS_Versao").unwrap().as_bytes()).unwrap()};
    println!("versao = {}", versao(buffer_raw, buffer_tamanho_reference));

    let c_string = unsafe { CString::from_raw(buffer_raw) };
    let resutl_string = c_string.into_string().unwrap();
    println!("buffer = len: {} :: string: {}", resutl_string.len(), resutl_string);
    println!("buffer_tamanho = {}", buffer_tamanho);

    let finalizar: Symbol<Finalizar> = unsafe { lib.get(CString::new("POS_Finalizar").unwrap().as_bytes()).unwrap() };
    println!("finalizar = {}", finalizar());

}

