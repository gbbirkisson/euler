const NUMBER: u64 = 600851475143;

pub fn solve() -> u64 {
    let factors = primes::factors_uniq(NUMBER);
    let res: u64 = *factors.last().unwrap();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let res = solve();
        println!("Answer: {}", res);
        assert_eq!(res, 6857);
    }
}
