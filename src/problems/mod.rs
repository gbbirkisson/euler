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
mod p011;
mod p012;
mod p013;
mod p014;
mod p015;
mod p016;
mod p017;
mod p018;
mod p019;
mod p020;
mod p021;
mod p067;

trait Solver<R> {
    fn solve(&self, id: u16) -> R;
}

impl<F, R> Solver<R> for F
where
    F: Fn() -> R,
    R: Debug,
{
    fn solve(&self, id: u16) -> R {
        println!("Problem: {id:03}");
        println!("Link: https://projecteuler.net/problem={id}");
        println!("Started!");
        let now = Instant::now();
        let res: R = self();
        let elapsed = now.elapsed().as_millis();
        println!("Done!");
        println!("Time taken: {elapsed} ms");
        println!("Answer: {res:?}");
        res
    }
}

pub fn run_solver(id: u16) {
    match id {
        1 => {
            p001::solver.solve(id);
        }
        2 => {
            p002::solver.solve(id);
        }
        3 => {
            p003::solver.solve(id);
        }
        4 => {
            p004::solver.solve(id);
        }
        5 => {
            p005::solver.solve(id);
        }
        6 => {
            p006::solver.solve(id);
        }
        7 => {
            p007::solver.solve(id);
        }
        8 => {
            p008::solver.solve(id);
        }
        9 => {
            p009::solver.solve(id);
        }
        10 => {
            p010::solver.solve(id);
        }
        11 => {
            p011::solver.solve(id);
        }
        12 => {
            p012::solver.solve(id);
        }
        13 => {
            p013::solver.solve(id);
        }
        14 => {
            p014::solver.solve(id);
        }
        15 => {
            p015::solver.solve(id);
        }
        16 => {
            p016::solver.solve(id);
        }
        17 => {
            p017::solver.solve(id);
        }
        18 => {
            p018::solver.solve(id);
        }
        19 => {
            p019::solver.solve(id);
        }
        20 => {
            p020::solver.solve(id);
        }
        21 => {
            p021::solver.solve(id);
        }
        67 => {
            p067::solver.solve(id);
        }
        _ => {
            panic!("Problem not solved yet")
        }
    };
}
