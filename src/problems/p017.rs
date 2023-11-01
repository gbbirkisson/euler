// https://projecteuler.net/problem=17

// If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are
// 3 + 3 + 5 + 4 + 4 = 19 letters used in total.

// If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many
// letters would be used?

// NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23
// letters and 115 (one hundred and fifteen) contains 20 letters. The use of "and" when writing out
// numbers is in compliance with British usage.

use std::ops::RangeInclusive;

fn to_words_str(number: &str) -> String {
    let len = number.len();
    match len {
        4 => {
            let mut res: String = String::new();
            res.push_str(&to_words_str(&number[0..1]));
            res.push_str(" thousand");
            res.push_str(&to_words_str(&number[1..]));
            res
        }
        3 => {
            let mut res: String = String::new();
            if &number[0..1] != "0" {
                res.push_str(&to_words_str(&number[0..1]));
                res.push_str(" hundred");
            }

            if &number[1..] != "00" {
                res.push_str(" and ");
                res.push_str(&to_words_str(&number[1..]));
            }

            res
        }
        2 => match number {
            "10" => "ten".to_string(),
            "11" => "eleven".to_string(),
            "12" => "twelve".to_string(),
            "13" => "thirteen".to_string(),
            "14" => "fourteen".to_string(),
            "15" => "fifteen".to_string(),
            "16" => "sixteen".to_string(),
            "17" => "seventeen".to_string(),
            "18" => "eighteen".to_string(),
            "19" => "nineteen".to_string(),
            _ => {
                let mut res: String = String::new();
                res.push_str(match &number[0..1] {
                    "2" => "twenty",
                    "3" => "thirty",
                    "4" => "forty",
                    "5" => "fifty",
                    "6" => "sixty",
                    "7" => "seventy",
                    "8" => "eighty",
                    "9" => "ninety",
                    _ => "",
                });
                if &number[1..] != "0" {
                    res.push('-');
                    res.push_str(&to_words_str(&number[1..]));
                }
                res
            }
        },
        1 => match number {
            "1" => "one".to_string(),
            "2" => "two".to_string(),
            "3" => "three".to_string(),
            "4" => "four".to_string(),
            "5" => "five".to_string(),
            "6" => "six".to_string(),
            "7" => "seven".to_string(),
            "8" => "eight".to_string(),
            "9" => "nine".to_string(),
            _ => "".to_string(),
        },
        _ => "".to_string(),
    }
}

fn to_words(number: u32) -> String {
    let number = format!("{:.0}", number);
    to_words_str(number.as_str())
}

fn count_letters(word: &str) -> usize {
    word.chars()
        .filter(|c| *c != ' ')
        .filter(|c| *c != '-')
        .count()
}

fn do_solve(range: RangeInclusive<u32>) -> usize {
    range
        .into_iter()
        .map(to_words)
        .map(|w| count_letters(w.as_str()))
        .sum()
}

pub fn solver() -> usize {
    do_solve(1..=1000)
}

#[cfg(test)]
mod tests {
    use crate::problems::Solver;

    use super::*;

    #[ignore]
    #[test]
    fn test_words() {
        assert_eq!(to_words(10), "ten");
        assert_eq!(to_words(100), "one hundred");
        assert_eq!(to_words(1000), "one thousand");
        assert_eq!(to_words(342), "three hundred and forty-two");
        assert_eq!(to_words(115), "one hundred and fifteen");
    }

    #[ignore]
    #[test]
    fn test_examples() {
        assert_eq!(do_solve(1..=5), 19);
        assert_eq!(count_letters(to_words(342).as_str()), 23);
        assert_eq!(count_letters(to_words(115).as_str()), 20);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solver.solve(17), 21124);
    }
}
