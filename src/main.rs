mod utils;
use crate::utils::generic_string::str_utils::get_length ;
use crate::utils::generic_string::math_utils::* ;
fn main() {
    let s = String::from("Hello World") ;
    println!("{}", get_length(&s)) ;
    println!("Hello, world!");
    let res = sieve(28) ;
    println!("{:?}", res) ;
    let n = 32 ;
    println!("{}",power_of_two(&n) ) ;
    println!("{}", n) ;
    
}
