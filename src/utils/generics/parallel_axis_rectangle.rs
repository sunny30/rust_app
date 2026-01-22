use std::any::Any;
use std::fmt::Display;
use std::ops::{Mul, Sub};

pub trait AreaOfRectangle<T: PartialOrd + Sub<Output = T> + Mul<Output = T> + Copy + Display> {
    fn area(&self) -> T;
}
#[derive(Debug)]
pub struct ParAxisRect<T: PartialOrd + Sub<Output = T> + Mul<Output = T> + Copy + Display> {
    pub x1: T,
    pub y1: T,
    pub x2: T,
    pub y2: T,
}

impl<T: PartialOrd + Sub<Output = T> + Mul<Output = T> + Copy + Display> AreaOfRectangle<T>
    for ParAxisRect<T>
{
    fn area(&self) -> T {
        let x = self.x2 - self.x1;
        let y = self.y2 - self.y1;
        x * y
    }
}

impl<T: PartialOrd + Sub<Output = T> + Mul<Output = T> + Copy + Display> PartialEq<Self>
    for ParAxisRect<T>
{
    fn eq(&self, other: &Self) -> bool {
        self.area() == other.area()
    }
}

impl<T: PartialOrd + Sub<Output = T> + Mul<Output = T> + Copy + Display> PartialOrd
    for ParAxisRect<T>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.area().partial_cmp(&other.area())
    }
}

impl<T: PartialOrd + Sub<Output = T> + Mul<Output = T> + Copy + Display> ParAxisRect<T> {
    pub fn new(x1: T, y1: T, x2: T, y2: T) -> Self {
        ParAxisRect { x1, y1, x2, y2 }
    }
}

fn check_i32_type<T: 'static>(value: &T) -> bool {
    std::any::TypeId::of::<T>() == std::any::TypeId::of::<i32>()
}

fn check_f32_type<T: 'static>(value: &T) -> bool {
    std::any::TypeId::of::<T>() == std::any::TypeId::of::<f32>()
}

fn check_f64_type<T: 'static>(value: &T) -> bool {
    std::any::TypeId::of::<T>() == std::any::TypeId::of::<f64>()
}

fn print_if_string(s: &(dyn Any + Send)) {
    if let Some(string) = s.downcast_ref::<String>() {
        println!("It's a string({}): '{}'", string.len(), string);
    } else {
        println!("Not a string...");
    }
}

pub fn pattern_matrix<T: PartialOrd + Sub<Output = T> + Mul<Output = T> + Copy + Display + 'static>(rect: &ParAxisRect<T>) -> String {
    match rect {  
        ParAxisRect{x1, ..} if check_i32_type(x1) => String::from("Integer coordinates"),
        ParAxisRect{x1, ..} if check_f32_type(x1) => String::from("Float coordinates"),
        ParAxisRect{x1, ..} if check_f64_type(x1) => String::from("Double coordinates"),
        _ => String::from("Non-integer coordinates")
    }
}
