use std::fmt::Debug;
use std::time::Instant;

pub fn run_problem<T: PartialEq + Debug>(problem_nr: u16, solver: fn() -> T, answer: Option<T>) {
    println!("Problem: {:03}", problem_nr);
    println!("Link: https://projecteuler.net/problem={}", problem_nr);
    println!("Started!");
    let now = Instant::now();
    let res = solver();
    let elapsed = now.elapsed().as_millis();
    println!("Done!");
    println!("Time taken: {} ms", elapsed);
    println!("Answer: {:?}", res);
    if let Some(answer) = answer {
        assert_eq!(res, answer);
    } else {
        println!("ANSWER NOT ASSERTED!");
    }
}
