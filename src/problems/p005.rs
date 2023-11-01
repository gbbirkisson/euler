// https://projecteuler.net/problem=5

// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any
// remainder. What is the smallest positive number that is evenly divisible by all of the numbers
// from 1 to 20?

const THRESHOLD: u64 = 20;

pub fn solver() -> u64 {
    let mut res: u64 = 0;
    for n in THRESHOLD.. {
        let mut is_match = true;
        for i in (2..THRESHOLD + 1).rev() {
            if n % i != 0 {
                is_match = false;
                break;
            }
        }

        if is_match {
            res = n;
            break;
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
        assert_eq!(solver.solve(5), 232792560);
    }
}
