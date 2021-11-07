// https://projecteuler.net/problem=18

// Find the maximum total from top to bottom of the triangle below:

// NOTE: As there are only 16384 routes, it is possible to solve this problem by trying every route. However, Problem 67, is the same challenge with a triangle containing one-hundred rows; it cannot be solved by brute force, and requires a clever method! ;o)

use std::cmp::Ordering;
use std::ops::Add;

const TRIANGLE: &str = "75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23";

struct Triangle {
    levels: Vec<Vec<usize>>,
}

struct Route {
    sum: usize,
    steps: Vec<(usize, usize)>,
}

impl PartialOrd for Route {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else {
            match self.sum > other.sum {
                true => Some(Ordering::Greater),
                false => Some(Ordering::Less),
            }
        }
    }
}

impl PartialEq<Self> for Route {
    fn eq(&self, other: &Self) -> bool {
        self.sum == other.sum
    }
}

impl Add for Route {
    type Output = Route;

    fn add(self, rhs: Self) -> Self::Output {
        let mut steps = self.steps;
        for s in rhs.steps {
            steps.push(s);
        }
        Route {
            sum: self.sum + rhs.sum,
            steps,
        }
    }
}

impl Triangle {
    fn new(str: &str) -> Triangle {
        let mut levels: Vec<Vec<usize>> = Vec::new();
        for (nr, line) in str.split('\n').into_iter().enumerate() {
            let nums: Vec<usize> = line
                .split(' ')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            assert_eq!(nums.len(), nr + 1);
            levels.push(nums);
        }
        Triangle { levels }
    }

    fn best_partial_route(
        &self,
        start_level: usize,
        start_index: usize,
        search_depth: usize,
    ) -> Option<Route> {
        // We are not searching further
        if search_depth == 0 {
            return None;
        }

        // Try to get this level
        let this_level = self.levels.get(start_level)?;
        let this_value = *this_level.get(start_index)?;
        let this_route = Route {
            sum: this_value,
            steps: vec![(start_index, this_value)],
        };

        // Try to get next level
        let left = self.best_partial_route(start_level + 1, start_index, search_depth - 1);
        let right = self.best_partial_route(start_level + 1, start_index + 1, search_depth - 1);

        // If left and right are not possible, return this route
        if left.is_none() || right.is_none() {
            return Some(this_route);
        }

        let left = left.unwrap();
        let right = right.unwrap();

        // Find the bigger one
        if left > right {
            Some(this_route + left)
        } else {
            Some(this_route + right)
        }
    }

    fn best_route(&self, branch_depth: usize) -> Route {
        // Branch depth need to be more than 0
        assert!(branch_depth > 0);

        // Let's fetch our root node
        let mut root = self
            .best_partial_route(0, 0, 1)
            .expect("we have a root node");

        // Start the loop
        let mut level = 0;
        let mut index = 0;
        loop {
            // Fetch left and right
            let partial_route_left = self.best_partial_route(level + 1, index, branch_depth);
            let partial_route_right = self.best_partial_route(level + 1, index + 1, branch_depth);

            // Return if nothing was found
            if partial_route_left.is_none() || partial_route_right.is_none() {
                return root;
            }
            let partial_route_left = partial_route_left.unwrap();
            let partial_route_right = partial_route_right.unwrap();

            // Add the better path to the root
            if partial_route_left > partial_route_right {
                root = root + partial_route_left;
            } else {
                root = root + partial_route_right;
            }

            // Update values for the next iteration
            level += branch_depth;
            index = root.steps.last().unwrap().0
        }
    }
}

pub fn solver() -> usize {
    let triangle = Triangle::new(TRIANGLE);
    let route = triangle.best_route(5);
    route.sum
}

#[cfg(test)]
mod tests {

    const TEST_TRIANGLE: &str = "3
7 4
2 4 6
8 5 9 3";

    use crate::problems::Solver;

    use super::*;

    #[ignore]
    #[test]
    fn test_small_triangle() {
        let triangle = Triangle::new(TEST_TRIANGLE);

        assert_eq!(triangle.best_route(1).sum, 23);
        assert_eq!(triangle.best_route(2).sum, 23);
        assert_eq!(triangle.best_route(3).sum, 23);
        assert_eq!(triangle.best_route(4).sum, 23);
        assert_eq!(triangle.best_route(5).sum, 23);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solver.solve(10), 1074);
    }
}
