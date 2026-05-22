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

pub fn get_node_at_index<'a, T:?Sized>(linklist: &'a mut SingleLinkedList<T>, index:i32)->*mut SingleLNode<T>{
    if index<0 || index>=linklist.len{
        panic!("wrong index at place")
    }else{
        let mut currentNode = linklist.head ;
       unsafe {
           for i in 0..index {
               currentNode = (*currentNode).next;
           }
       }
        currentNode
        /* `*mut SingleLNode<T>` value */
    }
}


pub fn drop_at_index<'a, T:?Sized>(linklist: &'a mut SingleLinkedList<T>, index:i32)->(){
    if index == 0 {
        let mut headNode = linklist.head ;
        unsafe {
            let mut nextNode = (*headNode).next;
            headNode = ptr::null_mut() ;
            linklist.head = nextNode ;
        }
        
    }else{
        let mut priorNode = get_node_at_index(linklist,index-1) ;
        unsafe {
            let mut indexNode = (*priorNode).next ;
            let mut nextNode = (*((*priorNode).next)).next ;
            (*priorNode).next = nextNode ;
            (*indexNode).next = ptr::null_mut() ;
            
            if(index == linklist.len-1){
                linklist.tail = priorNode ;
            }
        }
    }
    linklist.len-=1 ;
}


#[test]
fn single_link_list_test(){
    let mut list:SingleLinkedList<String> = SingleLinkedList::new() ;
    add_at_head(&mut list, Box::new(String::from("fist"))) ;
    add_at_head(&mut list, Box::new(String::from("new_fist"))) ;
    add_at_tail(&mut list, Box::new(String::from("tail"))) ;
    add_at_tail(&mut list, Box::new(String::from("new_tail"))) ;
    
    drop_at_index(&mut list, 3) ;
    let mut curr_head = list.head ;
    unsafe {
        for i in 0..list.len {
            println!("{:?}", (*curr_head).data);
            curr_head = (*curr_head).next;
        }

       // println!("{:?}", (*get_node_at_index(&mut list, 2)).data);
       // println!("{:?}", (*get_node_at_index(&mut list, 0)).data)
    }
}


