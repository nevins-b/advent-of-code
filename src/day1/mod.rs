use anyhow::Result;
use itertools::Itertools;
use nom::{AsChar, ToUsize};

static WORDY_DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn solve_part1() -> Result<usize> {
    let data = include_str!("input.txt");
    Ok(data
        .lines()
        .map(|line| {
            let digits = line
                .bytes()
                .filter(|c| c.is_dec_digit())
                .map(|c| c - b'0')
                .collect_vec();
            let s = digits.first().unwrap().to_usize();
            let e = digits.last().unwrap().to_usize();

            s * 10 + e
        })
        .sum())
}

fn next_digit(s: &str) -> Option<(usize, usize)> {
    let mut idx = usize::MAX;
    let mut digit = 0_usize;
    for (i, w) in WORDY_DIGITS.iter().enumerate() {
        match s.find(w) {
            None => continue,
            Some(pos) => {
                if pos < idx {
                    idx = pos;
                    digit = i;
                }
            }
        }
    }

    if let Some((pos, d)) = s
        .bytes()
        .enumerate()
        .filter(|c| c.1.is_ascii_digit())
        .map(|c| (c.0, c.1 - b'0'))
        .next()
        .map(|c| (c.0, c.1.to_usize()))
    {
        if pos < idx {
            idx = pos;
            digit = d;
        }
    }

    if idx == usize::MAX {
        None
    } else {
        Some((idx, digit))
    }
}

fn solve_part2() -> Result<usize> {
    let data = include_str!("input.txt");
    Ok(data
        .lines()
        .map(|mut line| {
            let mut digits = Vec::new();

            loop {
                match next_digit(line) {
                    None => break,
                    Some((idx, d)) => {
                        digits.push(d);
                        let start = idx + 1;
                        if start > line.len() {
                            break;
                        }
                        line = &line[start..];
                        if line.is_empty() {
                            break;
                        }
                    }
                }
            }

            let s = digits.first().unwrap().to_usize();
            let e = digits.last().unwrap().to_usize();

            let d = s * 10 + e;
            d
        })
        .sum())
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day1_part1() -> Result<()> {
        let s = solve_part1()?;
        println!("{}", s);
        assert_eq!(s, 55172);
        Ok(())
    }

    #[test]
    fn day1_part2() -> Result<()> {
        let s = solve_part2()?;
        println!("{}", s);
        assert_eq!(s, 54925);
        Ok(())
    }
}
