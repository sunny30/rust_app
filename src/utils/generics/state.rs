use std::fmt::{Display, Formatter, Pointer};
use std::ops::Deref;


pub struct Post<'a> {
    state: Option<Box<dyn PostState>>,
    actual_text: &'a String,
    reviewed_text: String
}

impl<'a> Display for Post<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Actual_Text: {}, Reviewed_Text: {}, ", self.actual_text, self.reviewed_text)
    }
}

impl<'a> Post<'a> {
    
    pub fn new(actual_text: &'a String) -> Self {
        Self {
            state: Some(Box::new(Draft{})),
            actual_text,
            reviewed_text: String::new()
        }
    }
    
    pub fn add_review(&mut self, review: &str){
        self.reviewed_text.push_str(review);
        self.state = Some(Box::new(Draft{})) ;
    }
    
    pub fn review_text(&mut self)  {
        if (&self).actual_text.len()  >3 {
            self.reviewed_text = self.actual_text.to_string();
            self.state = match self.state.take() {
                Some(_Box) => Some(_Box.review()),
                None => Some(Box::new(Draft {}))
            }
            
        }
    }
    
    
    
    
    
}

pub trait PostState{
    fn review(self: Box<Self>)->Box<dyn PostState>;
}

struct Draft{}

impl PostState for Draft {
    fn review(self: Box<Self>) -> Box<dyn PostState> {
        Box::new(ReviewedDraft {})
    }
}


struct ReviewedDraft{}

impl PostState for ReviewedDraft{
    fn review(self: Box<Self>) -> Box<dyn PostState>{
        Box::new(ReviewedDraft{})
    }
}

