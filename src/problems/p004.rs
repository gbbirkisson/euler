use std::ops::Range;

const RANGE: Range<usize> = 100..1000;

fn is_palindrome(num: usize) -> bool {
    let num_str = num.to_string();
    let rev_num_str: String = num_str.chars().rev().collect();
    rev_num_str == num_str
}

pub fn solver() -> usize {
    let mut res: usize = 0;

    for a in RANGE {
        for b in RANGE {
            let product = a * b;
            if is_palindrome(product) && res < product {
                res = product;
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::problems::Solver;

    #[test]
    fn test_solve() {
        assert_eq!(solver.solve(4), 906609);
    }
}
