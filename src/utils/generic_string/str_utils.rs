use std::fmt::Display;

pub fn get_length<'a, T: PartialOrd + Display + PartialEq>(s: &'a T) -> usize {
    return s.to_string().len();
}
