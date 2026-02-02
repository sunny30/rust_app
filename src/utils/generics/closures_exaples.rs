use std::ops::{Mul, Sub,Add};
use std::sync::LazyLock;

static DOT_PRODUCT: LazyLock<Box<dyn for<'a> Fn(&'a Vec<f64>, &'a Vec<f64>) -> f64 + Send + Sync>> = LazyLock::new(|| {
    Box::new(|a, b| a.iter().zip(b.iter()).map(|(&x, &y)| x * y).sum())
});


fn do_twice<T:Add<Output=T>+Mul<Output=T>+Sub<Output=T>+Sized+Copy+Clone, U:Fn(T)->T>(f:U, arg:T) ->T {
    return  f(arg)+f(arg) ;
    
}