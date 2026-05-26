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
    speak_thunk: Box<dyn Speak>,

}

impl<'a> AnythingSpeak<'a> {
    pub fn new<T:Speak+'static>(t: Box<T>)->Self{
        AnythingSpeak{
            _p: PhantomData,
            data: NonNull::from(t.as_ref()).cast(),
            speak_thunk: t,
        }
    }

    pub fn speak(&self){
        self.speak_thunk.speak()
    }
}

#[test]
fn raw_dyn_dispatch(){
    let mut a = AnythingSpeak::new(Box::new(Cat)) ;
    a.speak();
    a = AnythingSpeak::new(Box::new(Dog)) ;
    a.speak()

}