use std::cmp::Ordering;
use std::fmt::Display;
use std::hash::Hash;
use std::os::macos::raw::stat;

#[derive(Debug,Copy,Clone)]
pub struct QueueElement<T: Ord+Copy+Display+PartialEq+PartialOrd, U:Ord+Copy+Display+PartialEq+PartialOrd>  {
    priority: T,
    data: U
}

impl<T: Ord+Copy+Display+PartialEq+PartialOrd, U:Ord+Copy+Display+PartialEq+PartialOrd> Display for QueueElement<T,U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Priority: {}, Data: {}", self.priority, self.data)
    }
    
}


impl<T: Ord+Copy+Display+PartialEq+PartialOrd, U:Ord+Copy+Display+PartialEq+PartialOrd> QueueElement<T, U> {
    pub fn new(priority: T, data: U) -> Self {
        Self { priority, data }
    }
    
}



impl<T: Ord + Copy + Display + PartialEq + PartialOrd, U: Ord + Copy + Display + PartialEq+PartialOrd> Eq for QueueElement<T, U> {
   
    fn assert_receiver_is_total_eq(&self) {
        
    }
    
}





impl<T: Ord + Copy + Display + PartialEq + PartialOrd, U: Ord + Copy + Display + PartialOrd> PartialEq<Self> for QueueElement<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.data == other.data
    }
}




impl<T: Ord+Copy+Display+PartialEq+PartialOrd, U:Ord+Copy+Display+PartialOrd> Ord for QueueElement<T, U> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority).then_with(|| self.data.cmp(&other.data))
    }
}


impl<T: Ord+Copy+Display+PartialOrd, U:Ord+Copy+Display+PartialOrd> PartialOrd for QueueElement<T, U> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord+Copy+Display+PartialOrd, U:Ord+Copy+Display+PartialOrd> Hash for QueueElement<T, U> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) { 
        state.write(&*[self.priority.to_string().as_bytes(), self.data.to_string().as_bytes()].concat());
    }
}

pub trait NewTrait<T:Copy+Display+PartialEq+PartialOrd+Clone+Ord> {
    fn get_elem(t: &T) -> Box<Self> where Self: Sized;
}

impl<T:Copy+Display+PartialEq+PartialOrd+Clone+Ord> NewTrait<T> for QueueElement<T, T> {
    fn get_elem(t: &T) -> Box<QueueElement<T, T>> {
        Box::new(Self { priority: t.clone(), data: t.clone() })
    }
}

