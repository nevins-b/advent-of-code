use anyhow::Result;
use itertools::FoldWhile::{Continue, Done};
use itertools::{FoldWhile, Itertools};
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::{IResult, InputIter};
use num::Integer;
use std::collections::HashMap;

fn parse(input: &str) -> IResult<&str, (Vec<char>, HashMap<String, HashMap<char, String>>)> {
    let mut lines = input.lines();
    let directions = lines
        .next()
        .map(|l| l.iter_elements().collect_vec())
        .unwrap_or_default();
    lines.next();

    let mut m = HashMap::new();
    for l in lines {
        let (n, key) = alpha1(l)?;
        let (n, _) = tag(" = (")(n)?;
        let (n, left) = alpha1(n)?;
        let (n, _) = tag(", ")(n)?;
        let (_, right) = alpha1(n)?;
        m.insert(
            key.to_string(),
            vec![('L', left.to_string()), ('R', right.to_string())]
                .into_iter()
                .collect(),
        );
    }
    Ok((input, (directions, m)))
}

fn path_len(
    directions: &[char],
    map: &HashMap<String, HashMap<char, String>>,
    start: &str,
    predicate: impl Fn(&str) -> bool,
) -> u64 {
    let mut loc = start.to_string();
    let mut path = vec![start.to_string()];
    let l = directions
        .iter()
        .cycle()
        .fold_while(1_u64, |acc, dir| {
            let node = map.get(&loc).cloned().unwrap();
            loc = node.get(dir).cloned().unwrap();
            path.push(loc.to_string());
            if predicate(&loc) {
                Done(acc)
            } else {
                Continue(acc + 1)
            }
        })
        .into_inner();
    println!("nodes: {}, path: {}", l, path.join(" -> "));

    l
}
fn part1() -> Result<u64> {
    let (_, (directions, m)) = parse(include_str!("input.txt"))?;
    Ok(path_len(&directions, &m, "AAA", |x| x == "ZZZ"))
}

fn part2() -> Result<u64> {
    let (_, (directions, m)) = parse(include_str!("input.txt"))?;
    Ok(m.keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| path_len(&directions, &m, k, |x| x.ends_with('Z')))
        .sorted()
        .rev()
        .dedup()
        .fold(1, |acc, i| acc.lcm(&i)))
}

fn part2_the_ugly_way() -> Result<u64> {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn day8_part1() -> Result<()> {
        let s = part1()?;
        println!("{}", s);
        assert_eq!(s, 19667);
        Ok(())
    }

    #[test]
    fn day8_part2() -> Result<()> {
        let s = part2()?;
        println!("{}", s);
        assert_eq!(s, 19185263738117);
        Ok(())
    }
}
