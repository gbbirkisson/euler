// https://projecteuler.net/problem=14

// The following iterative sequence is defined for the set of positive integers:
// n → n/2 (n is even)
// n → 3n + 1 (n is odd)

// Using the rule above and starting with 13, we generate the following sequence:
// 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
// It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms.
// Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers
// finish at 1. Which starting number, under one million, produces the longest chain?

// NOTE: Once the chain starts the terms are allowed to go above one million.

const THRESHOLD: u64 = 1000000;

struct Collatz {
    curr: u64,
}

impl Collatz {
    fn new(start: u64) -> Self {
        Self { curr: start }
    }
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        match self.curr {
            0 => None,
            1 => {
                self.curr = 0;
                Some(1)
            }
            _ => {
                let res = self.curr;
                self.curr = match res % 2 {
                    0 => res / 2,
                    _ => 3 * res + 1,
                };
                Some(res)
            }
        }
    }
}

pub fn solver() -> u64 {
    let mut res: u64 = 0;
    let mut count: u64 = 0;

    for n in 0..THRESHOLD {
        let tmp: u64 = Collatz::new(n).count() as u64;
        if count < tmp {
            res = n;
            count = tmp;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::problems::Solver;

    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solver.solve(14), 837799);
    }
}
