use std::ptr ;
pub struct DoubleLNode<T: ?Sized>{
    pub data: Box<T>,
    pub next: *mut DoubleLNode<T>,
    pub prev: *mut DoubleLNode<T>
}


impl<T: ?Sized> DoubleLNode<T>{
    pub fn new(data:Box<T>)-> *mut Self{
        Box::into_raw(Box::new(
            DoubleLNode{
                data: data,
                next: ptr::null_mut(),
                prev: ptr::null_mut()
            }
        ))
    }
}


