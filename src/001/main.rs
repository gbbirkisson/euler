use util::run_problem;

const PROBLEM: u16 = 1;
const THRESHOLD: usize = 1000;

fn solver() -> usize {
    let res: usize = (0..THRESHOLD)
        .map(|x| match (x % 3, x % 5) {
            (0, _) => x,
            (_, 0) => x,
            (_, _) => 0
        })
        .sum();
    res
}

fn main() {
    run_problem(PROBLEM, solver, Some(233168));
}
