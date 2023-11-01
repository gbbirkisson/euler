// https://projecteuler.net/problem=9

// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which, a^2 + b^2 = c^2
// For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
// There exists exactly one Pythagorean triplet for which a + b + c = 1000. Find the product abc.

const TARGET: usize = 1000;

pub fn solver() -> usize {
    let mut res = (0, 0, 0);
    for a in 1..TARGET + 1 {
        for b in a + 1..TARGET + 1 {
            for c in b + 1..TARGET + 1 {
                if (a * a) + (b * b) == (c * c) && a + b + c == TARGET {
                    res = (a, b, c)
                }
            }
        }
    }
    res.0 * res.1 * res.2
}

#[cfg(test)]
mod tests {
    use crate::problems::Solver;

    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solver.solve(9), 31875000);
    }
}
