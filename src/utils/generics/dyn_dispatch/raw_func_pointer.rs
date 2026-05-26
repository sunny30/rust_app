use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct Dog ;

pub struct Cat ;
trait Speak{
    fn speak(&self){
        println!("abstract_noise")
    }
}

impl Speak for Dog{
    fn speak(&self){
        println!("woof_noise")
    }
}

impl Speak for Cat{
    fn speak(&self){
        println!("meow_noise")
    }
}

pub struct AnythingSpeak<'a>{
    _p : PhantomData<&'a()>,
    data: NonNull<()>,
    speak_thunk: unsafe fn(NonNull<()>),

}

impl<'a> AnythingSpeak<'a> {
    pub fn new<T:Speak>(t: &'a T)->Self{
        AnythingSpeak{
            _p: PhantomData,
            data: NonNull::from(t).cast(),
            speak_thunk: |data| unsafe{ data.cast::<T>().as_ref()}.speak(),
        }
    }

    pub fn speak(&self){
        unsafe {(self.speak_thunk)(self.data)}
    }
}

#[test]
fn raw_dyn_dispatch(){
    let mut a = AnythingSpeak::new(&Cat) ;
    a.speak();
    a = AnythingSpeak::new(&Dog) ;
    a.speak()
    
}