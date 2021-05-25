use util::run_problem;

const PROBLEM: u16 = 6;
const THRESHOLD: u64 = 100;

fn solver() -> u64 {
    let mut sum_of_squares: u64 = 0;
    let mut square_of_sums: u64 = 0;
    for n in 1..THRESHOLD + 1 {
        square_of_sums += n;
        sum_of_squares += n * n;
    }
    square_of_sums = square_of_sums * square_of_sums;
    square_of_sums - sum_of_squares
}

fn main() {
    run_problem(PROBLEM, solver, Some(25164150));
}
