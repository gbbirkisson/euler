use primes::{PrimeSet, Sieve};

const THRESHOLD: usize = 10001;

pub fn solve() -> u64 {
    let mut pset = Sieve::new();

    let mut res: u64 = 0;

    for n in pset.iter().take(THRESHOLD) {
        res = n;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let res = solve();
        println!("Answer: {}", res);
        assert_eq!(res, 104743);
    }
}
