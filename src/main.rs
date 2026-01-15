mod utils;
use crate::utils::generic_string::str_utils::get_length ;
fn main() {
    let s = String::from("Hello World") ;
    println!("{}", get_length(&s)) ;
    println!("Hello, world!");
}
