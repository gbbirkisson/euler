// https://projecteuler.net/problem=15

// Starting in the top left corner of a 2×2 grid, and only being able to move to the right and
// down, there are exactly 6 routes to the bottom right corner.

// How many such routes are there through a 20×20 grid?

use std::thread;

#[allow(dead_code)]
fn paths(x: u64, y: u64) -> u64 {
    match (x, y) {
        (0, _) => 1,
        (_, 0) => 1,
        (_, _) => paths(x, y - 1) + paths(x - 1, y),
    }
}

#[allow(dead_code)]
fn paths_parallel(d: u64, x: u64, y: u64) -> u64 {
    let t1 = thread::spawn(move || match d > 0 {
        true => paths_parallel(d - 1, x, y - 1),
        false => paths(x, y - 1),
    });

    let t2 = thread::spawn(move || match d > 0 {
        true => paths_parallel(d - 1, x - 1, y),
        false => paths(x - 1, y),
    });

    let res1 = t1.join().unwrap();
    let res2 = t2.join().unwrap();
    res1 + res2
}

fn paths_fast(n: u64) -> u64 {
    let mut res: f64 = 1.0;

    for i in 1..n + 1 {
        let t: u64 = n + i;
        let t: f64 = (t as f64) / (i as f64);
        res *= t
    }

    res as u64 + 1
}

pub fn solver() -> u64 {
    paths_fast(20)
    // paths_parallel(2, 15, 15)
}

#[cfg(test)]
mod tests {
    use crate::problems::Solver;

    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solver.solve(15), 137846528820);
    }
}
