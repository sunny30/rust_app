use std::ptr ;
use crate::utils::generics::data_structure::raw_single_ll::{add_at_head, add_at_tail, SingleLinkedList};

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


pub struct DoubleLinkList<T:?Sized>{
    len:i32,
    head: *mut DoubleLNode<T>,
    tail: *mut DoubleLNode<T>
}

impl<T:?Sized> DoubleLinkList<T> {
    
    fn new() -> Self{
        DoubleLinkList{
            len:0,
            head:ptr::null_mut(),
            tail: ptr::null_mut()
        }
    }
    
    pub fn add_at_front(&mut self, data:Box<T>) ->(){
        let newNode = DoubleLNode::new(data) ;
        let headNode = self.head ;
        unsafe {
            if (headNode == ptr::null_mut()) {
                self.head  = newNode ;
                self.tail = newNode ;
                self.len = 1 ;
            }else{
                (*newNode).next = headNode ;
                (*(headNode)).prev = newNode ;
                self.head = newNode ;
                self.len+=1 ;
            }
        }
    }
    
    
    pub fn add_at_tail(&mut self, data:Box<T>)->(){
        let newNode = DoubleLNode::new(data) ;
        let tailNode = self.tail ;
        unsafe {
            if(tailNode == ptr::null_mut()){
                self.head  = newNode ;
                self.tail = newNode ;
                self.len = 1 ;
            }else{
                (*newNode).prev = (*self).tail ;
                (*(self.tail)).next = newNode ;
                self.tail = newNode ;
                self.len+=1 ;
            }
        }
    }
}


#[test]
fn single_link_list_test(){
    let mut list:DoubleLinkList<String> = DoubleLinkList::new() ;
    list.add_at_front(Box::new(String::from("fist"))) ;
    list.add_at_front(Box::new(String::from("new_first")));
    list.add_at_tail(Box::new(String::from("tail")));
    list.add_at_tail(Box::new(String::from("new_tail"))) ;
    let mut curr_head = list.head ;
    for i in 0..list.len{
        unsafe {
            println!("{:?}", (*curr_head).data);
            curr_head = (*curr_head).next ;
        }
    }
}