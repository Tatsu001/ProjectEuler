fn common_divisor_vec(num: u64) -> Vec<u64> {
    let vec: Vec<u64> = (1..=num).filter(|x| num%x==0).collect();
    vec
}

fn is_prime_num(num: u64) -> bool {
    for n in 2..num {
        if num%n == 0 {
            return false
        }
    }
    return true
}

fn largest_prime_num_by(num: u64) -> u64 {
    let mut vec = common_divisor_vec(num);
    vec = vec.into_iter().filter(|x| is_prime_num(*x)).collect();
    vec[vec.len()-1]
}

fn main() {
    // let a = largest_prime_num_by(600851475143); // 600851475143  // 1億の桁で30秒くらいかかる // 40分経っても終わらない
    
    let num: u64 = 600851475143;
    let mut new_num: u64 = num;
    let mut largest_fact: u64 = 0;

    let mut counter: u64 = 2;
    
    while counter * counter <= new_num {
        if new_num % counter == 0 {
            new_num = new_num / counter;
            largest_fact = counter;
        }
        else {
            counter += 1;
        }
    }
    if new_num > largest_fact {
        largest_fact = new_num;
    }

    println!("{}", largest_fact);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_common_divisior() {
        assert_eq!(vec![1, 2, 3, 6], common_divisor_vec(6));
        assert_eq!(vec![1, 2, 13, 26], common_divisor_vec(26));
    }
    #[test]
    fn some_prime_number() {
        assert_eq!(true, is_prime_num(13));
        assert_eq!(false, is_prime_num(6));
    }
    #[test]
    fn some_largest_prime_num() {
        assert_eq!(3, largest_prime_num_by(6));
        assert_eq!(13, largest_prime_num_by(26));
    }
}
