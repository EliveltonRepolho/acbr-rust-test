### Installing
* Rust: https://doc.rust-lang.org/book/ch01-01-installation.html#installing-rustup-on-linux-or-macos

### Running this app
- rust app: `cargo run`
- rust app without having acbr lib in `/usr/lib: `RUSTFLAGS="-C link-args=-Wl,-rpath,$(pwd)/lib/acbr" cargo run`
- rust unit test: `cargo test`


cargo rustc -- --crate-type=cdylib

### Util references

Rust Conversions: https://nicholasbishop.github.io/rust-conversions/

ACBR Doc: https://acbr.sourceforge.io/ACBrLib/POS_Inicializar.html

ACBR Download LIB to `acbr_lib`: https://www.projetoacbr.com.br/forum/files/file/439-acbrlibposprinter/

Neon-bindings for electron: https://medium.com/nerd-for-tech/how-to-use-rust-inside-your-electron-application-using-neon-bindings-64bd83fec316 

https://snacky.blog/en/string-ffi-rust.html

https://stevedonovan.github.io/rustifications/2018/08/13/using-rust-like-c.html

https://doc.rust-lang.org/nomicon/ffi.html
