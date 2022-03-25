use libc::{c_int, size_t, c_char};
use std::ffi::CString;
use std::ptr;
use std::str;
use std::ffi::CStr;



macro_rules! next {
    ($p:expr) => { unsafe { $p = $p.add(1) } }
}

struct StrIter {
    p: *mut u8
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

fn str_iter(s: *mut u8) -> StrIter {
    StrIter {
        p: s
    }
}

#[link(name = "acbrposprinter64")]
extern {
    fn POS_Inicializar(
        eArqConfig: *const c_char,
        eChaveCrypt: *const c_char
    ) -> c_int;
    fn POS_Finalizar() -> c_int;
    fn POS_Versao(
        sVersao: *mut u8,
        esTamanho: *mut size_t
    ) -> c_int;
}

fn main() {

    let ini_file = CString::new( "abcr_lib/acbrlib.ini").unwrap();
    let chave_crypt = CString::new("").unwrap();

    unsafe {
        let mut ini_file_pointer: *const c_char = ini_file.as_ptr();
        let mut chave_crypt_pointer: *const c_char = chave_crypt.as_ptr();

        println!("POS_Inicializar exited with code: {}",
                 unsafe { POS_Inicializar(ini_file_pointer, chave_crypt_pointer) as i32 });

        let mut buffer_size = 0 as size_t;

        let mut buffer: Vec<u8> = Vec::with_capacity(255 as usize);
        let mut buffer_pointer: *mut u8 = buffer.as_mut_ptr();

        println!("POS_Versao exited with code: {}",
                 unsafe { POS_Versao(buffer_pointer, &mut buffer_size) as i32 });


        /*
        let s = match str::from_utf8(&buffer) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        println!("buffer_string: {}", s);
        */

        /*
        let c: String = str_iter(buffer_pointer).collect();
        println!("buffer_string = {}", c);
        */

        //for c in str_iter(buffer_pointer) { println!("{}",c); }

        println!("{:?}", chave_crypt_pointer);

        buffer_pointer = ptr::null_mut();
        ini_file_pointer = ptr::null();
        chave_crypt_pointer = ptr::null();
    }



}