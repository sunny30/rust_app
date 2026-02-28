use std::fmt::Display;
use std::ops::{Add, Mul};
#[derive(Debug, Clone)]
struct Coordinate<T:Add+Display+Mul+Clone>(T,T) ;

impl<T:Add<Output = T>+Display+Mul<Output = T>+Clone> Add for Coordinate<T> {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Coordinate(self.0+other.0,self.1+other.1)
    }
}

impl<T:Add<Output = T>+Display+Mul<Output = T>+Clone> Mul for Coordinate<T> {
    type Output = Self;
    
    fn mul(self, other: Self) -> Self {
        Coordinate(self.0*other.0,self.1*other.1)
    }
    
}


fn weight_add<T:Add<Output = T>+Display+Mul<Output = T>+Clone>(item1:Coordinate<T>,item2:Coordinate<T>, weight:T) -> Coordinate<T>{
    let mut res = (item1+item2) ;
    Coordinate(res.0*weight.clone(),res.1*weight) 
}


fn add_weighted_coordinate<T:Add<Output = T>+Display+Mul<Output = T>+Clone>(f:fn(Coordinate<T>,Coordinate<T>,T)->Coordinate<T>, item:Coordinate<T>, item2:Coordinate<T>,w1:T,w2:T) -> Coordinate<T>{
    f(item.clone(),item2.clone(),w1) + f(item,item2,w2)
}


#[cfg(test)]
mod tests {
    // Bring all items from the outer scope into the tests module's scope
    use super::*;

    #[test]
    fn coordinate_weight_tests() {
        
        let r1 = Coordinate(1,2) + Coordinate(2,3) ;
        let r2 = Coordinate(6,3) ;
        let res = add_weighted_coordinate(weight_add,r1,r2,2,3) ;
        println!("{:?}", res) ;
        
    }

    
}