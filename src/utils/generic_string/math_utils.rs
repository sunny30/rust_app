pub fn sieve(n:i32)-> Vec<i32>{
    let mut temp:Vec<i32>= vec![0; (n+2) as usize] ;
    for i in (2..=n).take_while(|&x| x*x<=n) {
        if temp[i as usize] == 0  {
            for mut j in (2..=n) {
                let index = i*j ;
                if index > n { break ; }
                temp[index as usize] = 1 ;
                //j = j+i ;
            }
        }
    }
    let mut result = Vec::new() ;
    for i in 2..=n {
        if temp[i as usize] == 0 {
            result.push(i) ;
        }
    }
    return result ;
    
}