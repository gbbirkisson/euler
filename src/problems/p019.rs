// https://projecteuler.net/problem=19

// You are given the following information, but you may prefer to do some research for yourself.

// * 1 Jan 1900 was a Monday.
// * Thirty days has September,
//   April, June and November.
//   All the rest have thirty-one,
//   Saving February alone,
//   Which has twenty-eight, rain or shine.
//   And on leap years, twenty-nine.
// * A leap year occurs on any year evenly divisible by 4, but not on a century unless it is
//   divisible by 400.

// How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31
// Dec 2000)?

// NOTES: I have decided to use the doomsday rule to figure this out:
//   https://en.wikipedia.org/wiki/Doomsday_rule

fn is_leap_year(year: isize) -> bool {
    match year % 4 {
        0 => match year % 100 {
            0 => year % 400 == 0,
            _ => true,
        },
        _ => false,
    }
}

fn century_anchor_day(year: isize) -> isize {
    match year {
        y if (1800..1900).contains(&y) => 5, // Friday
        y if (1900..2000).contains(&y) => 3, // Wednesday
        y if (2000..2100).contains(&y) => 2, // Tuesday
        y if (2100..2200).contains(&y) => 0, // Sunday
        _ => unreachable!(),
    }
}

fn year_anchor_day(year: isize) -> isize {
    let a = (year % 100) / 12;
    let b = (year % 100) % 12;
    let c = b / 4;

    (century_anchor_day(year) + a + b + c) % 7
}

fn month_doomsday(year: isize, month: isize) -> isize {
    let leap_year = is_leap_year(year);
    match month {
        1 if leap_year => 4,
        1 => 3,
        2 if leap_year => 1,
        2 => 7,
        3 => 7,
        4 => 4,
        5 => 2,
        6 => 6,
        7 => 4,
        8 => 1,
        9 => 5,
        10 => 3,
        11 => 7,
        12 => 5,
        _ => unreachable!(),
    }
}

fn weekday(year: isize, month: isize, day: isize) -> isize {
    let anchor_day = year_anchor_day(year);
    let dooms_day = month_doomsday(year, month);
    let days_between = day - dooms_day;
    ((anchor_day + days_between) + 28) % 7
}

pub fn solver() -> u32 {
    let mut count = 0;
    for year in 1901..=2000 {
        for month in 1..=12 {
            if weekday(year, month, 1) == 0 {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::problems::Solver;

    use super::*;

    #[ignore]
    #[test]
    fn test_weekday() {
        assert_eq!(weekday(2020, 11, 7), 6);
        assert_eq!(weekday(2021, 11, 10), 3);
        assert_eq!(weekday(2022, 1, 1), 6);
    }

    #[ignore]
    #[test]
    fn test_leap_year() {
        assert_eq!(is_leap_year(2020), true);
    }

    #[ignore]
    #[test]
    fn test_year_anchor_days() {
        assert_eq!(year_anchor_day(1978), 2);
        assert_eq!(year_anchor_day(1985), 4);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solver.solve(19), 171);
    }
}
