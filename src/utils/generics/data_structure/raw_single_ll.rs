use std::ptr;
pub struct SingleLNode<T: ?Sized>{
    pub data: Box<T>,
    next: *mut SingleLNode<T>
}


impl<T: ?Sized> SingleLNode<T>{
    fn new(data: Box<T>)-> *mut Self{
        Box::into_raw(Box::new(SingleLNode{
            data: data,
            next: ptr::null_mut()
            
        }))
    }
}

pub struct SingleLinkedList<T: ?Sized>{
    len: i32,
    head: *mut SingleLNode<T>,
    tail: *mut SingleLNode<T>
}

impl<T: ?Sized> SingleLinkedList<T>{
    fn new()-> Self{
        SingleLinkedList{
            len:0,
            head: ptr::null_mut(),
            tail: ptr::null_mut()
        }
    }
}

pub fn add_at_tail<'a, T:?Sized>(linklist: &'a mut SingleLinkedList<T>, data:Box<T>)->(){
    let tailNode = linklist.tail ;
    let newNode = SingleLNode::new(data) ;
    unsafe {
        (*tailNode).next = newNode;
        (*linklist).tail = newNode ;
    }
    
}


