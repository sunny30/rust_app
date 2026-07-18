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



    /// Get a shared (immutable) borrow of the inner value.
    /// Panics if the value is currently mutably borrowed.
    pub fn borrow(&self) -> Ref<'_, T> {
        let inner = unsafe { self.ptr.as_ref() };

        if inner.borrow_state < 0 {
            panic!("SharedMut: already mutably borrowed");
        }

        // Safety: we just checked it's not -1, safe to increment
        unsafe {
            (*self.ptr.as_ptr()).borrow_state += 1;
        }

        Ref { inner }
    }

    /// Get an exclusive (mutable) borrow of the inner value.
    /// Panics if the value is currently borrowed in any way.
    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        let inner = unsafe { self.ptr.as_ref() };

        if inner.borrow_state != 0 {
            panic!(
                "SharedMut: already borrowed (borrow_state = {})",
                inner.borrow_state
            );
        }

        // Safety: we just checked it's 0, safe to set -1
        unsafe {
            (*self.ptr.as_ptr()).borrow_state = -1;
        }

        RefMut { inner }
    }

    /// How many SharedMut handles point to this value right now.
    pub fn ref_count(&self) -> i32 {
        unsafe { self.ptr.as_ref().ref_count }
    }
}


pub struct Ref<'a, T> {
    inner: &'a Inner<T>,
}


pub struct RefMut<'a, T> {
    inner: &'a Inner<T>,
}


impl<T> Clone for SharedMut<T>{
    fn clone(&self) -> Self {
        unsafe {
            (*(self.ptr.as_ptr())).ref_count += 1; //reason it is working
            //NotNull return returns *mut Inner<T>, NonNull::as_ptr() always returns *mut T

        }
        SharedMut{
            ptr: self.ptr,
            _marker: PhantomData
        }
    }
}


impl<T> Drop for SharedMut<T>{
    fn drop(&mut self) {
        println!("Drop called from Shared Mut") ;
        unsafe {
            (*(self.ptr.as_ptr())).ref_count-=1 ;
            if (*self.ptr.as_ptr()).ref_count == 0 {
                // Reconstruct the Box → it drops T and frees heap memory
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
        

    }
}



