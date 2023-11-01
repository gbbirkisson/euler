// https://projecteuler.net/problem=2

// Each new term in the Fibonacci sequence is generated by adding the previous two terms. By
// starting with 1 and 2, the first 10 terms will be: 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ... By
// considering the terms in the Fibonacci sequence whose values do not exceed four million, find
// the sum of the even-valued terms.

const THRESHOLD: usize = 4000000;

struct Fib {
    curr: usize,
    next: usize,
}

impl Fib {
    fn new() -> Self {
        Fib { curr: 1, next: 1 }
    }
}

impl Iterator for Fib {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

pub fn solver() -> usize {
    let res: usize = Fib::new()
        .take_while(|x| x <= &THRESHOLD)
        .map(|x| match x % 2 {
            0 => x,
            _ => 0,
        })
        .sum();
    res
}

#[cfg(test)]
mod tests {
    use crate::problems::Solver;

    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solver.solve(2), 4613732);
    }
}
