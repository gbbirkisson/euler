const THRESHOLD: usize = 1000;

pub fn solver() -> usize {
    let res: usize = (0..THRESHOLD)
        .map(|x| match (x % 3, x % 5) {
            (0, _) => x,
            (_, 0) => x,
            (_, _) => 0,
        })
        .sum();
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::problems::Solver;

    #[test]
    fn test_solve() {
        assert_eq!(solver.solve(1), 233168);
    }
}
