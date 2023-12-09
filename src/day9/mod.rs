use itertools::Itertools;

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| l.split_whitespace().flat_map(|n| n.parse()).collect_vec())
        .collect_vec()
}

fn find_next(seq: Vec<i32>) -> i32 {
    seq.last()
        .map(|last| last + find_next(seq.iter().tuple_windows().map(|(x, y)| y - x).collect_vec()))
        .unwrap_or_default()
}

fn find_prev(seq: Vec<i32>) -> i32 {
    seq.first()
        .map(|first| {
            first
                - crate::day9::find_prev(
                    seq.iter().tuple_windows().map(|(x, y)| y - x).collect_vec(),
                )
        })
        .unwrap_or_default()
}

fn part1() -> i32 {
    let seqs = parse(include_str!("input.txt"));
    seqs.into_iter().map(find_next).sum()
}

fn part2() -> i32 {
    let seqs = parse(include_str!("input.txt"));
    seqs.into_iter().map(find_prev).sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;

    #[test]
    fn day9_part1() -> Result<()> {
        let s = part1();
        println!("{}", s);
        //assert_eq!(s, 114);
        assert!(s > 1417980928);
        Ok(())
    }

    #[test]
    fn day9_part2() -> Result<()> {
        let s = part2();
        println!("{}", s);
        assert!(s < 20658);
        Ok(())
    }
}
