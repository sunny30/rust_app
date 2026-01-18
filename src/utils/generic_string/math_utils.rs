pub fn sieve(n: i32) -> Vec<i32> {
    let mut temp: Vec<i32> = vec![0; (n + 2) as usize];
    for i in (2..=n).take_while(|&x| x * x <= n) {
        if temp[i as usize] == 0 {
            for mut j in (2..=n) {
                let index = i * j;
                if index > n {
                    break;
                }
                temp[index as usize] = 1;
                //j = j+i ;
            }
        }
    }
    let mut result = Vec::new();
    for i in 2..=n {
        if temp[i as usize] == 0 {
            result.push(i);
        }
    }
    return result;
}

pub fn power_of_two(n: &i32) -> bool {
    let mut x = *n;
    if (x & (x - 1) != 0) {
        return false;
    }
    return true;
}

pub fn power_of_p_in_fact(n: &i32, p: &i32) -> i32 {
    let mut result = 0;
    let mut n_cp = *n;
    let mut p_cp = *p;
    // result = n_cp/p_cp ;
    while n_cp > 0 {
        let tmp_res = n_cp / p_cp;
        result = result + tmp_res;
        p_cp = p_cp * (*p);
        if tmp_res == 0 {
            break;
        }
    }
    return result;
}
