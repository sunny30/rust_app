use crate::utils::generics::data_structure::safe_rc_mut::* ;
pub struct SingleLNode<T> {
    pub data: SharedMut<Box<T>>,      // SharedMut wrapping a Box
    next: *mut SingleLNode<T>,
}



