use std::fmt::Display;
use std::ops::{Mul, Sub};

pub trait AreaOfRectangle<T: PartialOrd+Sub<Output = T>+Mul<Output = T>+Copy+Display>{
    fn area(&self) -> T;
}
#[derive(Debug)]
pub struct ParAxisRect<T: PartialOrd+Sub<Output = T>+Mul<Output = T>+Copy+Display>{
    pub x1: T,
    pub y1:T,
    pub x2:T,
    pub y2:T
}

 impl<T:PartialOrd+Sub<Output = T>+Mul<Output = T> +Copy+Display> AreaOfRectangle<T> for ParAxisRect<T> {
    fn area(&self) -> T {
       let x = self.x2 - self.x1;
        let y = self.y2 - self.y1;
        x * y
    }
}

impl<T: PartialOrd + Sub<Output = T> + Mul<Output = T> +Copy+Display> PartialEq<Self> for ParAxisRect<T> {
    fn eq(&self, other: &Self) -> bool {
        self.area() == other.area()
    }
}

impl<T: PartialOrd+Sub<Output = T>+Mul<Output = T> 
+Copy+Display>  PartialOrd for ParAxisRect<T>{
     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.area().partial_cmp(&other.area())
    }
}

 impl<T: PartialOrd+Sub<Output = T>+Mul<Output = T>+Copy+Display> ParAxisRect<T>{
    pub fn new(x1:T,y1:T,x2:T,y2:T)->Self{
        ParAxisRect{x1,y1,x2,y2}
    }
}