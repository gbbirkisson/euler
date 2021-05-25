use util::run_problem;

const PROBLEM: u16 = 5;
const THRESHOLD: u64 = 20;

fn solver() -> u64 {
    let mut res: u64 = 0;
    for n in THRESHOLD.. {
        let mut is_match = true;
        for i in 2..THRESHOLD + 1 {
            if n % i != 0 {
                is_match = false;
            }
        }

        if is_match {
            res = n;
            break;
        }
    }
    res
}

fn main() {
    run_problem(PROBLEM, solver, Some(232792560));
}
