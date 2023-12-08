use anyhow::Result;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{multispace0, newline};
use nom::multi::separated_list1;
use nom::sequence::{delimited, tuple};
use nom::{combinator, IResult};

#[derive(Clone, Debug)]
struct Game {
    time: u64,
    distance: u64,
}

impl Game {
    fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }

    fn possibilities(&self) -> usize {
        (0..self.time)
            .filter_map(|p| {
                if (self.time - p) * p > self.distance {
                    Some(1)
                } else {
                    None
                }
            })
            .count()
    }
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, times) = delimited(
        tuple((tag("Time:"), multispace0)),
        separated_list1(multispace0, combinator::map(complete::u32, |v| v as u64)),
        newline,
    )(input)
    .map_err(|err| {
        println!("error parsing times: {}", err);
        err
    })?;

    let (input, distances) = delimited(
        tuple((tag("Distance:"), multispace0)),
        separated_list1(multispace0, combinator::map(complete::u32, |v| v as u64)),
        newline,
    )(input)
    .map_err(|err| {
        println!("error parsing distance: {}", err);
        err
    })?;

    Ok((
        input,
        (0..times.len())
            .map(|i| Game::new(times[i], distances[i]))
            .collect_vec(),
    ))
}

fn solve_part1() -> Result<usize> {
    let data = include_str!("input.txt");
    let (_, games) = parse_games(data)?;

    Ok(games.iter().map(|g| g.possibilities()).product())
}

fn solve_part2() -> Result<usize> {
    let g = Game::new(38677673, 234102711571236);
    Ok(g.possibilities())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day6_part1() -> Result<()> {
        let s = solve_part1()?;
        println!("{}", s);
        assert_ne!(s, 0);
        Ok(())
    }

    #[test]
    fn day6_part2() -> Result<()> {
        let s = solve_part2()?;
        println!("{}", s);
        assert_ne!(s, 0);
        Ok(())
    }
}
