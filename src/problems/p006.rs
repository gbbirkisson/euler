const THRESHOLD: u64 = 100;

pub fn solve() -> u64 {
    let mut sum_of_squares: u64 = 0;
    let mut square_of_sums: u64 = 0;
    for n in 1..THRESHOLD + 1 {
        square_of_sums += n;
        sum_of_squares += n * n;
    }
    square_of_sums = square_of_sums * square_of_sums;
    square_of_sums - sum_of_squares
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let res = solve();
        println!("Answer: {}", res);
        assert_eq!(res, 25164150);
    }
}
