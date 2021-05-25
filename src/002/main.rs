use util::run_problem;

const PROBLEM: u16 = 2;
const THRESHOLD: usize = 4000000;

struct Fib {
    curr: usize,
    next: usize,
}

impl Fib {
    fn new() -> Self {
        Fib { curr: 1, next: 1 }
    }
}

impl Iterator for Fib {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

fn solver() -> usize {
    let res: usize = Fib::new()
        .take_while(|x| x <= &THRESHOLD)
        .map(|x| match x % 2 {
            0 => x,
            _ => 0
        })
        .sum();
    res
}

fn main() {
    run_problem(PROBLEM, solver, Some(4613732))
}
