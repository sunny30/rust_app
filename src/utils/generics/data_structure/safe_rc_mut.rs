use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::ptr::NonNull;

struct Inner<T>{
    ref_count: i32,
    borrow_state:i32,
    value: UnsafeCell<T>
}


impl<T> Inner<T> {
    fn new(value: T) -> Self {
        Inner {
            ref_count: 1,
            borrow_state: 0,
            value: UnsafeCell::new(value),
        }
    }
}

pub struct SharedMut<T>{
    _marker: PhantomData<T>,
    ptr: NonNull<Inner<T>>
}




