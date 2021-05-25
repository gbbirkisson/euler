use primes::{PrimeSet, Sieve};

use util::run_problem;

const PROBLEM: u16 = 10;
const THRESHOLD: u64 = 2000000;

fn solver() -> u64 {
    let mut pset = Sieve::new();
    pset.iter().take_while(|x| x < &THRESHOLD).sum()
}

fn main() {
    run_problem(PROBLEM, solver, Some(142913828922));
}
