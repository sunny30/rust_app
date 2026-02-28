use std::ops::Add;

#[derive(Debug, Clone)]
struct Meters(u32) ;


#[derive(Debug, Clone)]
struct KiloMeters(u32) ;


impl Add<KiloMeters> for Meters {
    type Output = Meters;

    fn add(self, rhs: KiloMeters) -> Meters {
        Meters(self.0 + rhs.0 *  1000)
    }

}

impl Add<Meters> for KiloMeters {
    type Output = Meters;
    
    fn add(self, rhs: Meters) -> Meters {
        Meters(self.0 * 1000 + rhs.0)
    }
    
}

#[cfg(test)]
mod tests {
    // Bring all items from the outer scope into the tests module's scope
    use super::*;

    #[test]
    fn add_tests() {
        
        let m = Meters(2);
        let k = KiloMeters(3) ;
        let res = k.clone()+m.clone() ;
        let res1 = m+k ;
        
        println!("{:?}", res);
        print!("{:?}", res1);
    }
    
}