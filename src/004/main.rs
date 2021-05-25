use std::ops::Range;

use util::run_problem;

const PROBLEM: u16 = 4;
const RANGE: Range<usize> = 100..1000;

fn is_palindrome(num: usize) -> bool {
    let num_str = num.to_string();
    let rev_num_str: String = num_str.chars().rev().collect();
    return rev_num_str == num_str;
}

fn solver() -> usize {
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

fn main() {
    run_problem(PROBLEM, solver, Some(906609));
}
