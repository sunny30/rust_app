mod utils;

use std::collections::BinaryHeap;
use crate::utils::generic_string::math_utils::*;
use crate::utils::generic_string::str_utils::get_length;
use crate::utils::generics::parallel_axis_rectangle::*;
use crate::utils::generics::sort::sort;
use crate::utils::generics::matrices::* ;
use crate::utils::generics::priority_queue_struct::* ;

fn main() {
    let s = String::from("Hello World");
    println!("{}", get_length(&s));
    println!("Hello, world!");
    let res = sieve(28);
    println!("{:?}", res);
    let n = 28;
    let p1 = 2;
    let p2 = 3;
    println!("{}", power_of_p_in_fact(&n, &p1));
    println!("value of p1 {}", &p1);
    println!("{}", power_of_p_in_fact(&n, &p2));
    println!("{}", power_of_two(&n));
    println!("{}", n);
    let mut input1 = Vec::from(["he", "abc", "np"]);
    sort(&mut input1);
    println!("{:?}", input1);
    let mut input2 = Vec::from([1.33, 2.34, 6.67, 0.59]);
    sort(&mut input2);
    println!("{:?}", input2);
    let rect1 = ParAxisRect::new(1.2, 2.5, 4.6, 5.7);
    println!("{}", pattern_matrix(&rect1));
    let rect2 = ParAxisRect::new(3.3, 7.5, 9.6, 11.7);
    let rect3 = ParAxisRect::new(6.4, 2.5, 9.6, 3.7);
    let rect4 = ParAxisRect::new(4.3, 7.7, 5.6, 9.1);
    let mut input3 = Vec::from([rect1, rect2, rect3, rect4]);
    input3.sort_by(|x, y| x.area().partial_cmp(&y.area()).unwrap());
    println!("{:?}", input3);
    let matrix: Vec<Vec<i32>> = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9]
    ];
    let mat1 = Matrices::new(matrix) ;
    let mat2 = mat1.square_matrix() ;
    let mat3 = mat1.power(5) ;
    println!("{:?}", mat3) ;
    println!("{:?}", mat2) ;
    println!("{}", pattern_matrix(&input3.get(0).unwrap() ));
    let val1 = QueueElement::new(1,2) ;
    let val2 = QueueElement::new(2,3) ;
    let val3 = QueueElement::new(3,4);
    let lval3:QueueElement<i64,i64> = QueueElement::new(3,4);
    let nval3 = QueueElement::get_elem(&lval3) ;
    let nval = QueueElement::get_elem(&val1) ;
    let nval2 = QueueElement::get_elem(&lval3) ;
    let collection:Vec<Box<dyn NewTrait<QueueElement<_,_>>>> = vec![nval, nval2, nval3] ;
    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(val1);
    priority_queue.push(val2);
    priority_queue.push(val3) ;

    println!("{:?}", priority_queue) ; //max heap


}
