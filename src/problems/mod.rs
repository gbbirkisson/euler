use std::fmt::Debug;
use std::time::Instant;

mod p001;
mod p002;
mod p003;
mod p004;
mod p005;
mod p006;
mod p007;
mod p008;
mod p009;
mod p010;

struct Solver<T> {
    id: u16,
    solver: fn() -> T,
}

impl<T> Solver<T>
where
    T: Debug,
{
    fn run(self) {
        println!("Problem: {:03}", self.id);
        println!("Link: https://projecteuler.net/problem={}", self.id);
        println!("Started!");
        let now = Instant::now();
        let res: T = (self.solver)();
        let elapsed = now.elapsed().as_millis();
        println!("Done!");
        println!("Time taken: {} ms", elapsed);
        println!("Answer: {:?}", res);
    }
}

pub fn run_solver(id: u16) {
    match id {
        1 => Solver {
            id,
            solver: p001::solve,
        }
        .run(),
        2 => Solver {
            id,
            solver: p002::solve,
        }
        .run(),
        3 => Solver {
            id,
            solver: p003::solve,
        }
        .run(),
        4 => Solver {
            id,
            solver: p004::solve,
        }
        .run(),
        5 => Solver {
            id,
            solver: p005::solve,
        }
        .run(),
        6 => Solver {
            id,
            solver: p006::solve,
        }
        .run(),
        7 => Solver {
            id,
            solver: p007::solve,
        }
        .run(),
        8 => Solver {
            id,
            solver: p008::solve,
        }
        .run(),
        9 => Solver {
            id,
            solver: p009::solve,
        }
        .run(),
        10 => Solver {
            id,
            solver: p010::solve,
        }
        .run(),
        _ => todo!("Problem not solved yet"),
    };
}
