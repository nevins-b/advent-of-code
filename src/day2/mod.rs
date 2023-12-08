use anyhow::Result;
use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete, combinator, multi::separated_list1,
    sequence::separated_pair, IResult,
};
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Game {
    sets: Vec<HashMap<Color, usize>>,
    id: usize,
}

fn parse_color(line: &str) -> IResult<&str, Color> {
    nom::branch::alt((
        combinator::map(tag("red"), |_| Color::Red),
        combinator::map(tag("green"), |_| Color::Green),
        combinator::map(tag("blue"), |_| Color::Blue),
    ))(line)
}

impl Game {
    fn new(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("Game ")(input)?;
        let (input, id) = complete::u32(input)?;
        let (input, _) = tag(": ")(input)?;
        let (input, sets) = separated_list1(
            tag("; "),
            separated_list1(
                tag(", "),
                separated_pair(
                    combinator::map(complete::u32, |v| v as usize),
                    complete::space1,
                    parse_color,
                ),
            ),
        )(input)?;

        let sets = sets
            .into_iter()
            .map(|set| {
                let mut res = HashMap::new();
                for (n, c) in set {
                    res.entry(c).and_modify(|i| *i += n).or_insert(n);
                }
                res
            })
            .collect_vec();

        Ok((
            input,
            Game {
                sets,
                id: id as usize,
            },
        ))
    }

    fn power(&self) -> usize {
        let mut s: HashMap<Color, usize> = HashMap::new();

        for set in self.sets.iter() {
            for (c, n) in set {
                s.entry(*c).and_modify(|f| *f = (*f).max(*n)).or_insert(*n);
            }
        }
        s.values().product()
    }
}

pub fn solve_task1(file_content: &str) -> usize {
    let available: HashMap<Color, usize> =
        vec![(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]
            .into_iter()
            .collect();
    file_content
        .lines()
        .flat_map(|line| match Game::new(line) {
            Err(_) => None,
            Ok((_, g)) => Some(g),
        })
        .filter(|game| {
            available.iter().all(|(c, n)| {
                game.sets
                    .iter()
                    .all(|p| p.get(c).cloned().unwrap_or_default() <= *n)
            })
        })
        .map(|g| g.id)
        .sum()
}

pub fn solve_task2(file_content: &str) -> usize {
    file_content
        .lines()
        .flat_map(|line| match Game::new(line) {
            Err(_) => None,
            Ok((_, g)) => Some(g),
        })
        .map(|g| g.power())
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day2_part1() -> Result<()> {
        let s = solve_task1(include_str!("input.txt"));
        println!("{}", s);
        assert_eq!(s, 2283);
        Ok(())
    }

    #[test]
    fn day2_part2() -> Result<()> {
        let s = solve_task2(include_str!("input.txt"));
        println!("{}", s);
        assert_eq!(s, 78669);
        Ok(())
    }
}
