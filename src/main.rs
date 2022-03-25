use acbr_rust::{get_version};


fn main() {
    let version = get_version();
    println!("{}", version.unwrap());

}