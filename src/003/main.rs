use util::run_problem;

const PROBLEM: u16 = 3;
const NUMBER: u64 = 600851475143;

fn solver() -> u64 {
    let factors = primes::factors_uniq(NUMBER);
    let res: u64 = factors.last().unwrap().clone();
    res
}

fn main() {
    run_problem(PROBLEM, solver, Some(6857));
}
