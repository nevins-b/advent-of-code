use anyhow::Result;
use nom::sequence::separated_pair;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline, not_line_ending, space1},
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};
use std::ops::Range;

#[derive(Debug, Clone)]
struct TargetRange {
    src: usize,
    length: usize,
    dst: usize,
}

#[derive(Debug, Clone)]
struct ComponentRanges {
    // sorted in src ascending order
    ranges: Vec<TargetRange>,
}

#[derive(Debug, Clone)]
struct Mappings {
    maps: Vec<ComponentRanges>,
}

impl TargetRange {
    fn new(input: &str) -> IResult<&str, Self> {
        let (input, (dst, src, length)) = tuple((
            (terminated(complete::u32, space1)),
            (terminated(complete::u32, space1)),
            complete::u32,
        ))(input)?;
        Ok((
            input,
            Self {
                src: src as usize,
                dst: dst as usize,
                length: length as usize,
            },
        ))
    }

    fn contains_src(&self, src: usize) -> bool {
        self.src <= src && src < self.src + self.length
    }
    fn map(&self, src: usize) -> usize {
        self.dst + src - self.src
    }

    fn reverse_map(&self, dst: usize) -> usize {
        self.src + dst - self.dst
    }

    fn contains_dst(&self, dst: usize) -> bool {
        self.dst <= dst && dst < self.dst + self.length
    }
}

impl ComponentRanges {
    fn new(input: &str) -> IResult<&str, Self> {
        let (i, mut ranges) = separated_list0(newline, TargetRange::new)(input)?;
        ranges.sort_by_key(|r| r.src);
        Ok((i, Self { ranges }))
    }

    fn get(&self, src: usize) -> usize {
        match self.ranges.iter().find(|r| r.contains_src(src)) {
            Some(r) => r.map(src),
            None => src,
        }
    }
    fn ranges_srcs(&self) -> impl Iterator<Item = usize> + '_ {
        std::iter::once(0).chain(self.ranges.iter().map(|r| r.src))
    }
    fn reverse_get(&self, dst: usize) -> usize {
        match self.ranges.iter().find(|r| r.contains_dst(dst)) {
            Some(r) => r.reverse_map(dst),
            None => dst,
        }
    }
}

impl Mappings {
    fn new(input: &str) -> IResult<&str, Self> {
        let (i, maps) = separated_list1(
            tuple((newline, newline)),
            preceded(tuple((not_line_ending, newline)), ComponentRanges::new),
        )(input)?;
        Ok((i, Self { maps }))
    }

    fn get(&self, seed: usize) -> usize {
        self.maps.iter().fold(seed, |v, m| m.get(v))
    }

    fn get_src(&self, dst: usize, applied_levels: usize) -> usize {
        let mut value = dst;
        for m in self.maps.iter().take(applied_levels).rev() {
            value = m.reverse_get(value);
        }
        value
    }

    fn get_from_level(&self, src: usize, already_applied: usize) -> usize {
        self.maps[already_applied..]
            .iter()
            .fold(src, |v, m| m.get(v))
    }

    fn enumerate(&self) -> impl Iterator<Item = (usize, &ComponentRanges)> + '_ {
        self.maps.iter().enumerate()
    }
}

fn task1_seeds(data: &str) -> IResult<&str, Vec<usize>> {
    delimited(
        tag("seeds: "),
        separated_list1(space1, map(complete::u32, |x| x as usize)),
        tuple((newline, newline)),
    )(data)
}

fn task2_seeds(data: &str) -> IResult<&str, Vec<Range<usize>>> {
    delimited(
        tag("seeds: "),
        separated_list1(
            space1,
            map(
                separated_pair(complete::u32, space1, complete::u32),
                |(x, y)| (x as usize)..((x + y) as usize),
            ),
        ),
        tuple((newline, newline)),
    )(data)
}
pub fn solve_task1() -> Result<usize> {
    let data = include_str!("input.txt");
    let (i, seeds) = task1_seeds(data).expect("failed to parse inputs");
    let (_, components) = Mappings::new(i)?;
    Ok(seeds.into_iter().map(|s| components.get(s)).min().unwrap())
}

pub fn solve_task2() -> Result<usize> {
    let data = include_str!("input.txt");
    let (i, seeds) = task2_seeds(data).expect("failed to parse inputs");
    let (_, components) = Mappings::new(i)?;

    Ok(components
        .enumerate()
        .map(|(idx, range)| {
            range
                .ranges_srcs()
                .filter(|&start| {
                    let src = components.get_src(start, idx);
                    seeds.iter().any(|r| r.contains(&src))
                })
                .map(|start| components.get_from_level(start, idx))
                .min()
                .unwrap()
        })
        .min()
        .unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn day5_part1() -> Result<()> {
        let result = solve_task1()?;
        println!("{}", result);
        assert_eq!(result, 178159714);
        Ok(())
    }

    #[test]
    fn day5_part2() -> Result<()> {
        let result = solve_task2()?;
        println!("{}", result);
        assert_eq!(result, 100165128);
        Ok(())
    }
}
