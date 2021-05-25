const THRESHOLD: u64 = 20;

pub fn solver() -> u64 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::problems::Solver;

    #[test]
    fn test_solve() {
        assert_eq!(solver.solve(5), 232792560);
    }
}
