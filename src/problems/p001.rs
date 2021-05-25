const THRESHOLD: usize = 1000;

pub fn solve() -> usize {
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

    #[test]
    fn test_solve() {
        let res = solve();
        println!("Answer: {}", res);
        assert_eq!(res, 233168);
    }
}
