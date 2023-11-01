// https://projecteuler.net/problem=16

// 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.

// What is the sum of the digits of the number 2^1000?

const RADIX: u32 = 10;

pub fn solver() -> u32 {
    let x = 2.0_f64;
    let x = x.powi(1000);
    let x = format!("{:.0}", x);
    x.chars().map(|x| x.to_digit(RADIX).unwrap()).sum()
}

#[cfg(test)]
mod tests {
    use crate::problems::Solver;

    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solver.solve(16), 1366);
    }
}
