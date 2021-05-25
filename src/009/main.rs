use util::run_problem;

const PROBLEM: u16 = 9;
const MAX: usize = 1000;
const TARGET: usize = 1000;

fn solver() -> usize {
    let mut res = (0, 0, 0);
    for a in 1..MAX + 1 {
        for b in a + 1..MAX + 1 {
            for c in b + 1..MAX + 1 {
                if (a * a) + (b * b) == (c * c) && a + b + c == TARGET {
                    res = (a, b, c)
                }
            }
        }
    }
    res.0 * res.1 * res.2
}

fn main() {
    run_problem(PROBLEM, solver, Some(31875000));
}
