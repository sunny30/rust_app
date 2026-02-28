use std::cmp::Ordering;
use std::fmt::Display;
use std::hash::Hash;
use std::os::macos::raw::stat;

#[derive(Debug, Copy, Clone)]
pub struct QueueElement<
    T: Ord + Copy + Display + PartialEq + PartialOrd,
    U: Ord + Copy + Display + PartialEq + PartialOrd,
> {
    priority: T,
    data: U,
}

impl<
    T: Ord + Copy + Display + PartialEq + PartialOrd,
    U: Ord + Copy + Display + PartialEq + PartialOrd,
> Display for QueueElement<T, U>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Priority: {}, Data: {}", self.priority, self.data)
    }
}

impl<
    T: Ord + Copy + Display + PartialEq + PartialOrd,
    U: Ord + Copy + Display + PartialEq + PartialOrd,
> QueueElement<T, U>
{
    pub fn new(priority: T, data: U) -> Self {
        Self { priority, data }
    }
}

impl<
    T: Ord + Copy + Display + PartialEq + PartialOrd,
    U: Ord + Copy + Display + PartialEq + PartialOrd,
> Eq for QueueElement<T, U>
{
    fn assert_receiver_is_total_eq(&self) {}
}

pub trait AbstractElement {
    fn can_write(&self) -> bool;
}
#[derive(Debug, Copy, Clone)]
pub struct Element<'a, T: Display> {
    priority: &'a T,
}

#[derive(Debug, Copy, Clone)]
pub struct AnothElement<'a, T: Display> {
    p_value: &'a T,
}

impl<'a, T: Display> Element<'a, T> {
    pub(crate) fn new(priority: &'a T) -> Self
    where
        Self: Sized,
    {
        Self { priority }
    }
}

impl<'a, T: Display> AnothElement<'a, T> {
    pub(crate) fn new(p_value: &'a T) -> Self
    where
        Self: Sized,
    {
        Self { p_value }
    }
}

impl<'a, T: Display> AbstractElement for AnothElement<'a, T> {
    fn can_write(&self) -> bool {
        false
    }
}

impl<'a, T: Display> AbstractElement for Element<'a, T> {
    fn can_write(&self) -> bool {
        true
    }
}

impl<T: Ord + Copy + Display + PartialEq + PartialOrd, U: Ord + Copy + Display + PartialOrd>
    PartialEq<Self> for QueueElement<T, U>
{
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.data == other.data
    }
}

impl<T: Ord + Copy + Display + PartialEq + PartialOrd, U: Ord + Copy + Display + PartialOrd> Ord
    for QueueElement<T, U>
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority
            .cmp(&other.priority)
            .then_with(|| self.data.cmp(&other.data))
    }
}

impl<T: Ord + Copy + Display + PartialOrd, U: Ord + Copy + Display + PartialOrd> PartialOrd
    for QueueElement<T, U>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord + Copy + Display + PartialOrd, U: Ord + Copy + Display + PartialOrd> Hash
    for QueueElement<T, U>
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(
            &*[
                self.priority.to_string().as_bytes(),
                self.data.to_string().as_bytes(),
            ]
            .concat(),
        );
    }
}

pub trait NewTrait<T: Copy + Display + PartialEq + PartialOrd + Clone + Ord> {
    fn get_elem(t: &T) -> Box<Self>
    where
        Self: Sized;
}

impl<T: Copy + Display + PartialEq + PartialOrd + Clone + Ord> NewTrait<T> for QueueElement<T, T> {
    fn get_elem(t: &T) -> Box<QueueElement<T, T>> {
        Box::new(Self {
            priority: t.clone(),
            data: t.clone(),
        })
    }
}

pub fn print_message(c: Box<&dyn AbstractElement>) -> String {
    if c.can_write() {
        "Can write".to_string()
    } else {
        "Can't write".to_string()
    }
}
