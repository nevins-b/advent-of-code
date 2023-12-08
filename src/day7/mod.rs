use crate::day7::HandType::{
    FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair,
};
use anyhow::Result;
use itertools::Itertools;
use nom::InputIter;
use once_cell::sync::Lazy;
use std::cmp::Ordering;
use std::collections::HashMap;

static CARD_RANKS: Lazy<HashMap<char, usize>> = Lazy::new(|| {
    vec![
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('J', 11),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
        ('J', 1),
    ]
    .into_iter()
    .collect()
});

#[derive(Clone, Debug, Default, PartialEq, Eq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    #[default]
    HighCard,
}

impl Into<usize> for &HandType {
    fn into(self) -> usize {
        match self {
            FiveOfAKind => 6,
            FourOfAKind => 5,
            FullHouse => 4,
            ThreeOfAKind => 3,
            TwoPair => 2,
            OnePair => 1,
            HighCard => 0,
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        let i: usize = self.into();
        i.cmp(&other.into())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Hand {
    cards: Vec<usize>,
    bid: u64,
    rank: HandType,
}

impl Hand {
    fn new(input: &str, part2: bool) -> Result<Self> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        let mut h = Self {
            bid: parts[1].parse()?,
            ..Self::default()
        };
        h.cards = parts[0]
            .iter_elements()
            .flat_map(|c| CARD_RANKS.get(&c))
            .cloned()
            .collect_vec();

        if part2 {
            h.rank_part2();
        } else {
            h.rank_part1();
        }

        Ok(h)
    }

    fn rank_part1(&mut self) {
        let mut counts = HashMap::new();
        for i in self.cards.iter() {
            counts.entry(*i).and_modify(|e| *e += 1).or_insert(1);
        }
        self.rank = match counts.values().max().unwrap() {
            5 => FiveOfAKind,
            4 => FourOfAKind,
            3 => {
                if counts.values().any(|c| *c == 2) {
                    FullHouse
                } else {
                    ThreeOfAKind
                }
            }
            2 => {
                if counts.values().filter(|c| (*c).eq(&2)).count() == 2 {
                    TwoPair
                } else {
                    OnePair
                }
            }
            1 => HighCard,
            _ => unreachable!("can't have this count of cards"),
        }
    }

    fn rank_part2(&mut self) {
        let mut counts = HashMap::new();
        for i in self.cards.iter() {
            counts.entry(*i).and_modify(|e| *e += 1).or_insert(1);
        }
        let joker_count = counts.get(&1).cloned().unwrap_or_default();
        self.rank = match counts.values().max().unwrap() {
            5 => FiveOfAKind,
            4 => {
                if joker_count > 0 {
                    FiveOfAKind
                } else {
                    FourOfAKind
                }
            }
            3 => {
                if counts.values().any(|c| *c == 2) {
                    match joker_count {
                        3 | 2 => FiveOfAKind,
                        1 => FourOfAKind,
                        _ => FullHouse,
                    }
                } else {
                    match joker_count {
                        1 | 3 => FourOfAKind,
                        _ => ThreeOfAKind,
                    }
                }
            }
            2 => {
                if counts.values().filter(|c| (*c).eq(&2)).count() == 2 {
                    match joker_count {
                        2 => FourOfAKind,
                        1 => FullHouse,
                        _ => TwoPair,
                    }
                } else if joker_count > 0 {
                    ThreeOfAKind
                } else {
                    OnePair
                }
            }
            1 => {
                if joker_count > 0 {
                    OnePair
                } else {
                    HighCard
                }
            }
            _ => unreachable!("can't have this count of cards"),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.rank.cmp(&other.rank) {
            Ordering::Equal => {
                for i in 0..self.cards.len() {
                    match self.cards[i].cmp(&other.cards[i]) {
                        Ordering::Equal => continue,
                        o => {
                            return o;
                        }
                    }
                }
                Ordering::Equal
            }
            o => o,
        }
    }
}

fn solve(part2: bool) -> Result<u64> {
    let data = include_str!("input.txt");
    let hands = data
        .lines()
        .flat_map(|l| Hand::new(l, part2))
        .sorted()
        .collect_vec();
    let score = hands
        .iter()
        .enumerate()
        .map(|(i, h)| h.bid * (i + 1) as u64)
        .sum();
    Ok(score)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day7_part1() -> Result<()> {
        let s = solve(false)?;
        println!("{}", s);
        Ok(())
    }

    #[test]
    fn day7_part2() -> Result<()> {
        let s = solve(true)?;
        println!("{}", s);
        Ok(())
    }
}
