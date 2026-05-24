use std::marker::PhantomData;
use std::ptr ;
use crate::utils::generics::data_structure::raw_single_ll::{add_at_head, add_at_tail, get_node_at_index, SingleLinkedList};

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

    pub fn get_node_at_index(&mut self, index:i32)->*mut DoubleLNode<T>{
        if index<0 || index>=self.len{
            panic!("wrong index at place")
        }else{
            let mut currentNode = self.head ;
            unsafe {
                for i in 0..index {
                    currentNode = (*currentNode).next;
                }
            }
            currentNode
            /* `*mut SingleLNode<T>` value */
        }
    }

    pub fn drop_at_index(&mut self, index:i32)->(){
        if index == 0 {
            let mut headNode = self.head ;
            unsafe {
                let mut nextNode = (*headNode).next;
                (*nextNode).prev = ptr::null_mut() ;
                (*headNode).next = ptr::null_mut() ;
                (*headNode).prev = ptr::null_mut() ;
                self.head = nextNode ;
            }

        }else{
            let mut priorNode = self.get_node_at_index(index-1) ;
            unsafe {
                let mut indexNode = (*priorNode).next ;
                let mut nextNode = (*((*priorNode).next)).next ;
                (*priorNode).next = nextNode ;
                if  nextNode != ptr::null_mut() {
                    (*nextNode).prev = priorNode;
                }
                (*indexNode).next = ptr::null_mut() ;
                (*indexNode).prev = ptr::null_mut() ;

                if(index == self.len-1){
                    self.tail = priorNode ;
                }
            }
        }
        self.len-=1 ;
    }


    pub fn iter(&self)->Iter<T>{
        Iter {
            current:   self.head,
            remaining: self.len,
            _marker:   PhantomData,
        }
    }


}



pub struct Iter<'a, T: ?Sized>{
    current: *mut DoubleLNode<T>,
    remaining: i32,
    _marker: PhantomData<& 'a T>
}

impl<'a, T:?Sized>Iterator for Iter<'a,T > {
    type Item = &'a DoubleLNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
       if self.current == std::ptr::null_mut() {
           None
       }else{
           unsafe {
               let mut node = & (*(self.current)) ;
               self.current = (*(self.current)).next ;
               self.remaining -=1 ;
               return Some(node) ;
               
           }
       }
    }
}




// impl<'a, T:?Sized> Iterator for DoubleLinkList<T>{
//     type Item = *mut DoubleLNode<T>;
//
//
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.len <=0 {
//             None
//         }else{
//
//         }
//     }
// }


#[test]
fn single_link_list_test(){
    let mut list:DoubleLinkList<String> = DoubleLinkList::new() ;
    list.add_at_front(Box::new(String::from("fist"))) ;
    list.add_at_front(Box::new(String::from("new_first")));
    list.add_at_tail(Box::new(String::from("tail")));
    list.add_at_tail(Box::new(String::from("new_tail"))) ;
   // list.drop_at_index(2) ;
    //let mut curr_head = list.head ;
    // for i in 0..list.len{
    //     unsafe {
    //         println!("{:?}", (*curr_head).data);
    //         curr_head = (*curr_head).next ;
    //     }
    // }
    for d in list.iter(){
        println!("{:?}", d.data) ;
    }
}