use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
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

impl<T> SharedMut<T>{
    pub fn new(value:T)->Self{
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
        let mut inner = unsafe { self.ptr.as_ref() };

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


//Ref<'a, T> — guard returned by borrow()
//
// Lifetime 'a ties this guard to the SharedMut that created it.
// You cannot drop the SharedMut while a Ref is alive (borrow checker enforces).
// On drop: decrement borrow_state.
// ─────────────────────────────────────────────────────────────────────────────


impl<T> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        // Safety: borrow_state > 0 means no mutable borrow exists.
        // UnsafeCell::get gives *mut T; we cast to &T which is safe here
        // because no &mut T can coexist (enforced by borrow_state check).
        unsafe { &*self.inner.value.get() }
    }
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        // Safety: inner is still valid (SharedMut alive because 'a borrow)
        unsafe {
            let inner = self.inner as *const Inner<T> as *mut Inner<T>;  ;
            (*inner).ref_count-=1
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// RefMut<'a, T> — guard returned by borrow_mut()
//
// Same idea as Ref but exclusive. On drop: reset borrow_state to 0.
// ─────────────────────────────────────────────────────────────────────────────


impl<T> Deref for RefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.inner.value.get() }
    }
}

impl<T> DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        // Safety: borrow_state == -1 means we are the sole mutable borrower.
        // No other &T or &mut T exists, so this is sound.
        unsafe { &mut *self.inner.value.get() }
    }
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        unsafe {
            let state = (self.inner as *const Inner<T> as *mut Inner<T>);
            (*state).borrow_state = 0;
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_shared_ownership() {
        let a = SharedMut::new(42);
        let b = a.clone();
        let c = b.clone();

        assert_eq!(a.ref_count(), 3);
        assert_eq!(*a.borrow(), 42);
        assert_eq!(*b.borrow(), 42);
        assert_eq!(*c.borrow(), 42);
    }

    #[test]
    fn mutation_visible_to_all_handles() {
        let a = SharedMut::new(0);
        let b = a.clone();

        // mutate through `a`
        *a.borrow_mut() = 99;

        // visible through `b` — same heap allocation
        assert_eq!(*b.borrow(), 99);
    }

    #[test]
    fn multiple_shared_borrows_allowed() {
        let a = SharedMut::new(String::from("hello"));
        let b = a.clone();

        let r1 = a.borrow();
        let r2 = b.borrow(); // fine — both are shared borrows

        assert_eq!(*r1, "hello");
        assert_eq!(*r2, "hello");
        // r1 and r2 dropped here → borrow_state back to 0
    }

    #[test]
    fn ref_count_drops_correctly() {
        let a = SharedMut::new(10);
        assert_eq!(a.ref_count(), 1);

        let b = a.clone();
        assert_eq!(a.ref_count(), 2);

        drop(b);
        assert_eq!(a.ref_count(), 1);
        // memory freed when `a` drops at end of scope
    }

    #[test]
    #[should_panic(expected = "already mutably borrowed")]
    fn shared_borrow_while_mutably_borrowed_panics() {
        let a = SharedMut::new(1);
        let _m = a.borrow_mut();
        let _s = a.borrow(); // should panic
    }

    #[test]
    #[should_panic(expected = "already borrowed")]
    fn mutable_borrow_while_shared_borrowed_panics() {
        let a = SharedMut::new(1);
        let _s = a.borrow();
        let _m = a.borrow_mut(); // should panic
    }

    #[test]
    #[should_panic(expected = "already borrowed")]
    fn double_mutable_borrow_panics() {
        let a = SharedMut::new(1);
        let _m1 = a.borrow_mut();
        let _m2 = a.borrow_mut(); // should panic
    }

    #[test]
    fn borrow_released_after_guard_drop() {
        let a = SharedMut::new(vec![1, 2, 3]);

        {
            let mut m = a.borrow_mut();
            m.push(4);
        } // RefMut dropped here → borrow_state = 0

        // can borrow again now
        assert_eq!(*a.borrow(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn works_with_heap_allocated_t() {
        let a = SharedMut::new(Box::new(String::from("deep")));
        let b = a.clone();

        assert_eq!(**b.borrow(), "deep");
    }

    #[test]
    fn borrow_mut_changes_value() {
        let a = SharedMut::new(10);
        let b = a.clone();

        *a.borrow_mut() = 99;  // reassign through mutable guard

        assert_eq!(*b.borrow(), 99);  // visible through other handle
    }
}


