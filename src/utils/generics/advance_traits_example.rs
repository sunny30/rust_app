use std::ops::Add;

#[derive(Debug, Clone)]
struct Meters(u32);

#[derive(Debug, Clone)]
struct KiloMeters(u32);

impl Add<KiloMeters> for Meters {
    type Output = Meters;

    fn add(self, rhs: KiloMeters) -> Meters {
        Meters(self.0 + rhs.0 * 1000)
    }
}

impl Add<Meters> for KiloMeters {
    type Output = Meters;

    fn add(self, rhs: Meters) -> Meters {
        Meters(self.0 * 1000 + rhs.0)
    }
}

trait Pilot {
    fn fly(&self) -> String;
    fn call();
}

trait Driver {
    fn fly(&self) -> String;
    fn call();
}

struct Human;

impl Human {
    pub fn fly(&self) -> String {
        "I am Human air for firing ".to_string()
    }

    pub fn call() {
        println!("Human calling")
    }
}

impl Pilot for Human {
    fn fly(&self) -> String {
        "Captain is ready to take off ".to_string()
    }

    fn call() {
        println!("Captain calling")
    }
}

impl Driver for Human {
    fn fly(&self) -> String {
        "I am Driver, speed is flying ".to_string()
    }

    fn call() {
        println!("Driver calling")
    }
}

#[cfg(test)]
mod tests {
    // Bring all items from the outer scope into the tests module's scope
    use super::*;

    #[test]
    fn add_tests() {
        let m = Meters(2);
        let k = KiloMeters(3);
        let res = k.clone() + m.clone();
        let res1 = m + k;

        println!("{:?}", res);
        print!("{:?}", res1);
    }

    #[test]
    fn trait_tests() {
        let person = Human;
        println!("{}", person.fly());
        println!("{}", Pilot::fly(&person));
        println!("{}", Driver::fly(&person));

        Human::call();
        <Human as Pilot>::call();
        <Human as Driver>::call();
    }
}
