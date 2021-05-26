// https://projecteuler.net/problem=12

// The sequence of triangle numbers is generated by adding the natural numbers. So the 7th triangle number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28. The first ten terms would be:

// 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...

// Let us list the factors of the first seven triangle numbers:

// 1: 1
// 3: 1,3
// 6: 1,2,3,6
// 10: 1,2,5,10
// 15: 1,3,5,15
// 21: 1,3,7,21
// 28: 1,2,4,7,14,28

// We can see that 28 is the first triangle number to have over five divisors.
// What is the value of the first triangle number to have over five hundred divisors?

// TIPS on solution: https://www.quora.com/What-is-the-fastest-way-to-find-the-divisors-of-a-number
// 600=2^3×3^1×5^2
// The exponents of the prime factorization of 600 are 3, 1, and 2, so the number of
// divisors of 600, including the ones containing exponents that are zero, are (3+1)(1+1)(2+1)=24.

use primes::factors;

const THRESHOLD: u64 = 500;

struct Tri {
    pos: u64,
    sum: u64,
}

impl Tri {
    fn new() -> Self {
        Self { pos: 1, sum: 0 }
    }
}

impl Iterator for Tri {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.sum += self.pos;
        self.pos += 1;
        Some(self.sum)
    }
}

fn get_number_of_divisors(n: u64) -> u64 {
    let mut res: u64 = 1;
    let mut c: u64 = 0;
    let mut e: u64 = 0;

    for p in factors(n) {
        if p != e {
            res *= c + 1;
            c = 0;
            e = p;
        }
        c += 1;
    }
    res *= c + 1;

    res
}

pub fn solver() -> u64 {
    for i in Tri::new() {
        if get_number_of_divisors(i) > THRESHOLD {
            return i;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use crate::problems::Solver;

    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solver.solve(12), 76576500);
    }
}
