fn even_sum_with_fibo(n: i32) -> i32 {
    let mut n_old = 1;
    let mut n_new = 2;

    let mut temp = 0i32;
    let mut even_sum = 2i32;

    while temp < n {

        if temp%2 == 0 {
            even_sum += temp;
        }

        temp = n_old + n_new; // 3 5 8
        n_old = n_new; // 2 3 5
        n_new = temp; // 3 5 8

    }

    even_sum

}

fn main() {
    let even_sum = even_sum_with_fibo(4000000);
    println!("{}", even_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_sum() {
        assert_eq!(10, even_sum_with_fibo(10));
        assert_eq!(44, even_sum_with_fibo(50));
    }
}
