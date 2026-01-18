pub fn sort<T:PartialOrd+Copy>(input: & mut Vec<T>)->(){
    
    input.sort_by(|a,b| a.partial_cmp(b).unwrap())

}