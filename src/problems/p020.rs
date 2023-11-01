// https://projecteuler.net/problem=20

// n! means n × (n − 1) × ... × 3 × 2 × 1

// For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
// and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.

// Find the sum of the digits in the number 100!

use num::BigUint;

const RADIX: u32 = 10;

fn solve(n: u64) -> u32 {
    let mut x = BigUint::from(1_u64);
    for n in 2..=n {
        x *= BigUint::from(n);
    }
    let x = format!("{:.0}", x);
    x.chars().map(|x| x.to_digit(RADIX).unwrap()).sum()
}

pub fn solver() -> u32 {
    solve(100)
}

#[cfg(test)]
mod tests {
    use crate::problems::Solver;

    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(solve(10), 27)
    }

    #[test]
    fn test_solve() {
        assert_eq!(solver.solve(20), 648);
    }
}
