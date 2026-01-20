use std::fmt::Display;
use std::ops::{Add, Mul};

#[derive(Debug)]
pub struct Matrices<T:Add<Output=T>+ Mul<Output=T>+Copy+Display>{
    pub matrix:Vec<Vec<T>>,
    pub row:i32,
    pub col:i32
}

impl<T:Add<Output=T>+ Mul<Output=T>+Copy+Display> Matrices<T> {
    pub fn new(matrix:Vec<Vec<T>>) -> Self{
        let row = matrix.len() as i32 ;
        let col =  matrix[0].len() as i32 ;
        Self{matrix: matrix, row, col}
    }
    
   pub fn multiply(&self, other:&Matrices<T>) -> Matrices<T>{
       let mut result = self.matrix.clone();
        if (self.row == other.row ) && (self.col == other.col) {
            for i in (0 .. other.row) {
                for j in (0 .. other.col) {
                 let mut temp_result:T = self.matrix[i as usize][0].clone() * other.matrix[0][j as usize].clone();
                 for k in (1 .. self.col) {
                     temp_result = temp_result + self.matrix[i as usize][k as usize].clone() * other.matrix[k as usize][j as usize].clone();
                 }
                 result[i as usize][j as usize] = temp_result;
                }
            }
        }
       return  Matrices{matrix:result, row:other.row, col:other.col};
   }
    
   pub fn square_matrix(&self) -> Matrices<T>{
       self.multiply(&self)
   } 
    
    
}