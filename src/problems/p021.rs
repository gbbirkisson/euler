// https://projecteuler.net/problem=21

// Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly
// into n). If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a
// and b are called amicable numbers.

// For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110;
// therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.

// Evaluate the sum of all the amicable numbers under 10000.

trait ProperDivisors {
    fn proper_divisors(&self) -> Option<Vec<u64>>;
}

impl ProperDivisors for u64 {
    fn proper_divisors(&self) -> Option<Vec<u64>> {
        if self.le(&1) {
            return None;
        }
        let mut divisors: Vec<u64> = Vec::new();

        for i in 1..*self {
            if *self % i == 0 {
                divisors.push(i);
            }
        }
        Option::from(divisors)
    }
}

fn d(n: u64) -> Option<u64> {
    n.proper_divisors().map(|x| x.iter().sum())
}

pub fn solver() -> u64 {
    let mut res = Vec::new();
    for a in 2..10000 {
        if let Some(da) = d(a) {
            let b = da;
            if let Some(db) = d(b) {
                if da == b && db == a && a != b {
                    res.push(a);
                }
            }
        }
    }
    res.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::problems::Solver;

    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solver.solve(21), 31626);
    }
}
