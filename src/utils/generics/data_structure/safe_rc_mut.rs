use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::ptr::NonNull;
use crate::utils::generics::priority_queue_struct::Element;

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

impl<T> SharedMut<T>{
    pub fn new(&self, value:T)->Self{
        let inner = Inner::new(value) ;
        let boxed = Box::new(inner) ;
        let raw_ptr = Box::into_raw(boxed) ;
        SharedMut{
            ptr: unsafe{NonNull::new_unchecked(raw_ptr)},
            _marker:PhantomData
        }
    }
}


pub struct Ref<'a, T> {
    inner: &'a Inner<T>,
}


pub struct RefMut<'a, T> {
    inner: &'a Inner<T>,
}


