fn sum_of_multiples_of_3_or_5(n: i32) -> i32 {
    (1..n).filter(|x| x%3 == 0 || x%5 == 0)
          .sum()
}

fn main() {
    let sum: i32 = sum_of_multiples_of_3_or_5(1000);

    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_sum() {
        assert_eq!(23, sum_of_multiples_of_3_or_5(10));
        assert_eq!(45, sum_of_multiples_of_3_or_5(13));
    }
}