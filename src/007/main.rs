use primes::{PrimeSet, Sieve};

use util::run_problem;

const PROBLEM: u16 = 7;
const THRESHOLD: usize = 10001;

fn solver() -> u64 {
    let mut pset = Sieve::new();

    let mut res: u64 = 0;

    for n in pset.iter().take(THRESHOLD) {
        res = n;
    }

    res
}

fn main() {
    run_problem(PROBLEM, solver, Some(104743));
}
