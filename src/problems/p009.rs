const TARGET: usize = 1000;

pub fn solver() -> usize {
    let mut res = (0, 0, 0);
    for a in 1..TARGET + 1 {
        for b in a + 1..TARGET + 1 {
            for c in b + 1..TARGET + 1 {
                if (a * a) + (b * b) == (c * c) && a + b + c == TARGET {
                    res = (a, b, c)
                }
            }
        }
    }
    res.0 * res.1 * res.2
}

#[cfg(test)]
mod tests {
    use crate::problems::Solver;

    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solver.solve(9), 31875000);
    }
}
