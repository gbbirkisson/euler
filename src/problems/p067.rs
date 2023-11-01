// https://projecteuler.net/problem=67

// Find the maximum total from top to bottom in triangle.txt (right click and 'Save Link/Target
// As...'), a 15K text file containing a triangle with one-hundred rows.

// NOTE: This is a much more difficult version of Problem 18. It is not possible to try every route
// to solve this problem, as there are 299 altogether! If you could check one trillion (1012)
// routes every second it would take over twenty billion years to check them all. There is an
// efficient algorithm to solve it. ;o)

use crate::problems::p018::Triangle;
use std::fs;

pub fn solver() -> usize {
    let triangle = fs::read_to_string("./data/p067_triangle.txt").expect("file should be there");
    let triangle = Triangle::new(triangle.as_str());
    let route = triangle.best_route(13);
    route.get_sum()
}

#[cfg(test)]
mod tests {
    use crate::problems::Solver;

    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solver.solve(67), 7273);
    }
}
