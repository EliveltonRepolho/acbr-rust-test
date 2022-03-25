extern crate libloading;

use std::ffi::{CString};

use libloading::{Library, Symbol};

use std::os::raw::c_char;

type Inicializar = extern "system" fn(eArqConfig: *const c_char, eChaveCrypt: *const c_char) -> i32;
type Finalizar = extern "system" fn() -> i32;
type Versao = extern "C" fn(sVersao: *const u8, esTamanho: *const i32) -> i32;

macro_rules! next {
    ($p:expr) => { unsafe { $p = $p.add(1) } }
}

struct StrIter {
    p: *const u8
}

impl Iterator for StrIter {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let ch = unsafe { *self.p };
        if ch == 0 { // string is finished
            None
        } else {
            next!(self.p);
            Some(ch as char)
        }
    }
}

fn str_iter(s: *const u8) -> StrIter {
    StrIter {
        p: s
    }
}

fn main() {

    for c in str_iter(b"hello\0".as_ptr()) {
        println!("{}",c);
    }

    let s: String = str_iter(b"hello\0".as_ptr()).collect();
    println!("s: {}", s);

    let ini_file = CString::new( "abcr_lib/acbrlib.ini").unwrap();
    let ini_file_pointer = ini_file.as_ptr();

    let chave_crypt = CString::new("").unwrap();
    let chave_crypt_pointer = chave_crypt.as_ptr();

    let library_path = "abcr_lib/libacbrposprinter64.so";
    println!("Loading acbr from {}", library_path);

    let lib = unsafe { Library::new(library_path).unwrap() };

    let inicializar: Symbol<Inicializar> = unsafe { lib.get(CString::new("POS_Inicializar").unwrap().as_bytes()).unwrap() };
    println!("inicializar = {}", inicializar(ini_file_pointer, chave_crypt_pointer));

    let src = b"                                                                                                                                                                                                                                                               \0";
    let p = src.as_ptr();

    let mut buffer_size: i32 = 0;
    let buffer_size_reference = &mut buffer_size;

    let versao: Symbol<Versao> = unsafe { lib.get(CString::new("POS_Versao").unwrap().as_bytes()).unwrap()};
    println!("versao = {}", versao(p, buffer_size_reference));

    for c in str_iter(p) {
        println!("{}",c);
    }
    let s: String = str_iter(b"hello\0".as_ptr()).collect();

    //let resutl_string = c_string.into_string().unwrap();
    //println!("buffer_string = len: {} :: string: {}", resutl_string.len(), resutl_string);
    println!("buffer_size = {}", buffer_size);

    let finalizar: Symbol<Finalizar> = unsafe { lib.get(CString::new("POS_Finalizar").unwrap().as_bytes()).unwrap() };
    println!("finalizar = {}", finalizar());

}

