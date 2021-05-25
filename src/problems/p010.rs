use primes::{PrimeSet, Sieve};

const THRESHOLD: u64 = 2000000;

pub fn solve() -> u64 {
    let mut pset = Sieve::new();
    pset.iter().take_while(|x| x < &THRESHOLD).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let res = solve();
        println!("Answer: {}", res);
        assert_eq!(res, 142913828922);
    }
}
