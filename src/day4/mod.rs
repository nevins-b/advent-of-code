use itertools::Itertools;

fn parse_number(input: &str) -> Vec<u32> {
    input
        .split_ascii_whitespace()
        .flat_map(|n| {
            if let Ok(i) = n.parse::<u32>() {
                Some(i)
            } else {
                None
            }
        })
        .collect_vec()
}
fn str_to_card_matches(line: &str) -> Option<usize> {
    let (_card_title, numbers) = line.split_once(':')?;

    let (win_numbers_str, given_numbers_str) = numbers.split_once('|')?;

    let winning = parse_number(win_numbers_str);

    Some(
        parse_number(given_numbers_str)
            .into_iter()
            .filter(|c| winning.contains(c))
            .count(),
    )
}

fn solve_task1(file_content: &str) -> usize {
    file_content
        .lines()
        .flat_map(str_to_card_matches)
        .filter(|matches| *matches > 0)
        .map(|matches| 1 << (matches - 1))
        .sum()
}

fn solve_task2(file_content: &str) -> usize {
    let matches = file_content
        .lines()
        .flat_map(str_to_card_matches)
        .collect::<Vec<_>>();
    let mut cards_instances = vec![1; matches.len()];
    let mut sum = 0;
    for (i, matches) in matches.into_iter().enumerate() {
        let j0 = i + 1;
        let j1 = usize::min(cards_instances.len(), i + matches + 1);
        for j in j0..j1 {
            cards_instances[j] += cards_instances[i];
        }
        sum += cards_instances[i];
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    #[test]
    fn day4_part1() -> Result<()> {
        let s = solve_task1(include_str!("input.txt"));
        println!("{}", s);
        assert_eq!(s, 21558);
        Ok(())
    }

    #[test]
    fn day4_part2() -> Result<()> {
        let s = solve_task2(include_str!("input.txt"));
        println!("{}", s);
        assert_eq!(s, 10425665);
        Ok(())
    }
}
