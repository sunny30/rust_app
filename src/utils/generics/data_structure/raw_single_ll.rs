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
    len:  i32,
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
        if(tailNode.is_null()){
            (*linklist).head = newNode;
            (*linklist).tail = newNode;
            (*linklist).len = (*linklist).len + 1;
        }else {
            (*tailNode).next = newNode;
            (*linklist).tail = newNode;
            (*linklist).len = (*linklist).len + 1;
        }
    }
    
}

pub fn add_at_head<'a, T:?Sized>(linklist: &'a mut SingleLinkedList<T>, data:Box<T>)->(){
    let mut headNode = linklist.head ;
    let mut newNode = SingleLNode::new(data) ;

    unsafe {
        if(headNode.is_null()){
            (*linklist).head = newNode;
            (*linklist).tail = newNode;
            (*linklist).len = (*linklist).len + 1;
        }else {
            (*newNode).next = headNode ;
            (*linklist).head = newNode;
            (*linklist).len = (*linklist).len + 1;
        }
    }
}


#[test]
fn single_link_list_test(){
    let mut list:SingleLinkedList<String> = SingleLinkedList::new() ;
    add_at_head(&mut list, Box::new(String::from("fist"))) ;
    add_at_head(&mut list, Box::new(String::from("new_fist"))) ;
    add_at_tail(&mut list, Box::new(String::from("tail"))) ;
    add_at_tail(&mut list, Box::new(String::from("new_tail"))) ;
    let mut curr_head = list.head ;
    for i in 0..list.len{
        unsafe { 
            println!("{:?}", (*curr_head).data); 
            curr_head = (*curr_head).next ;
        }
    }
}


