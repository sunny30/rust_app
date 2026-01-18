mod utils;
use crate::utils::generic_string::str_utils::get_length ;
use crate::utils::generic_string::math_utils::* ;
fn main() {
    let s = String::from("Hello World") ;
    println!("{}", get_length(&s)) ;
    println!("Hello, world!");
    let res = sieve(28) ;
    println!("{:?}", res) ;
    let n = 28 ;
    let p1 = 2 ;
    let p2 = 3 ;
    println!("{}", power_of_p_in_fact(&n, &p1)) ;
    println!("value of p1 {}", &p1) ;
    println!("{}", power_of_p_in_fact(&n, &p2)) ;
    println!("{}",power_of_two(&n) ) ;
    println!("{}", n) ;

}
