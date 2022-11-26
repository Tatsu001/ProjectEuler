fn check_palindromic(num: &u32) -> bool {
    let num_str = num.to_string();
    let vec: Vec<u32> = num_str.chars().map(|x| x as u32).collect();
    
    let mut iter_len = vec.len();
    let mut fin = vec.len() - 1;

    if vec.len()%2 != 0 { // odd (digit)
        iter_len -= 1;
    }

    if iter_len == 2 { // In the case of 3 digits, the following loop cannot be turned
        if vec[0] == vec[fin] {
            return true
        }
        return false
    }

    for n in 0..(iter_len/2) {
        if vec[n] == vec[fin] {
            fin -= 1;
        }
        else {
            return false
        }
    }
    return true
}

fn max_palindromic(digit: &u32) -> u32 {
    let n1 = 10 * (digit-1);
    let n2 = n1.clone();
    let fin_iter = 10u32.pow(*digit);
    let mut max_num = 0;
    
    for i in n1..fin_iter {
        for j in n2..fin_iter {
            if check_palindromic(&(i*j)) && max_num < i*j {
                max_num = i*j;
            }
        }
    }
    max_num
}

fn main() {
    let a = max_palindromic(&3);
    println!("{}", a);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_palindromic() {
        assert_eq!(true, check_palindromic(&1255521));
        assert_eq!(true, check_palindromic(&987789));
        assert_eq!(false, check_palindromic(&32874));
        assert_eq!(false, check_palindromic(&432));
        assert_eq!(false, check_palindromic(&9409));
    }

    #[test]
    fn some_max_palindromic() {
        assert_eq!(9009, max_palindromic(&2));
    }
}