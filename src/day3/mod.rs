use anyhow::Result;
use itertools::Itertools;
use nom::AsChar;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::ops::RangeInclusive;

#[derive(Clone, Default, Eq, PartialEq)]
struct SchematicLine {
    line_number: usize,
    symbol_position: HashSet<usize>,
    gear_position: HashSet<usize>,
    part_numbers: HashMap<usize, String>,
}

impl Hash for SchematicLine {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.line_number);
    }
}
impl SchematicLine {
    fn new(n: usize, input: &str) -> Result<Self> {
        let mut s = SchematicLine::default();
        s.line_number = n;
        let mut number_pos = 0;
        let mut p = Vec::new();
        for (idx, b) in input.bytes().enumerate() {
            if b.is_dec_digit() {
                if p.is_empty() {
                    number_pos = idx;
                }
                p.push(b - b'0');
            } else {
                if !p.is_empty() {
                    let n = p.to_vec().iter().join("");
                    s.part_numbers.insert(number_pos, n);
                    p = Vec::new();
                }
                if b != b'.' {
                    s.symbol_position.insert(idx);
                    if b == b'*' {
                        s.gear_position.insert(idx);
                    }
                }
            }
        }
        if !p.is_empty() {
            let n = p.to_vec().iter().join("");
            s.part_numbers.insert(number_pos, n);
        }

        Ok(s)
    }

    fn touches_part_numbers(&self, idx: usize) -> Vec<String> {
        self.part_numbers
            .clone()
            .into_iter()
            .filter(|(start_index, n)| {
                let start = if *start_index == 0 {
                    0
                } else {
                    *start_index - 1
                };
                let end = *start_index + n.len();
                (start..=end).contains(&idx)
            })
            .map(|(_, n)| n.clone())
            .collect_vec()
    }

    fn touches_gear(&self, idx: RangeInclusive<usize>) -> Vec<usize> {
        idx.into_iter()
            .filter(|i| self.gear_position.contains(i))
            .collect_vec()
    }
    fn is_symbol(&self, idx: RangeInclusive<usize>) -> bool {
        idx.into_iter().any(|i| self.symbol_position.contains(&i))
    }
}
struct Schematic {
    lines: Vec<SchematicLine>,
}

impl Schematic {
    fn new(input: &str) -> Self {
        Self {
            lines: input
                .lines()
                .enumerate()
                .flat_map(|(n, l)| SchematicLine::new(n, l))
                .collect_vec(),
        }
    }

    fn part_numbers(&self) -> Result<Vec<u32>> {
        let mut parts = Vec::new();
        for (idx, line) in self.lines.iter().enumerate() {
            for (pos, s) in line.part_numbers.iter() {
                let start = if *pos == 0 { 0 } else { *pos - 1 };
                let end = pos + s.len();
                let positions = start..=end;
                let mut target_lines = vec![Some(line), self.lines.get(idx + 1)];
                if idx > 0 {
                    target_lines.push(self.lines.get(idx - 1));
                }
                if target_lines
                    .iter()
                    .flatten()
                    .any(|l| l.is_symbol(positions.clone()))
                {
                    parts.push(s.parse()?);
                } else {
                    println!("{}", s);
                }
            }
        }
        Ok(parts)
    }

    fn get_target_lines(&self, line_number: usize) -> Vec<&SchematicLine> {
        let mut target_lines = vec![self.lines.get(line_number), self.lines.get(line_number + 1)];
        if line_number > 0 {
            target_lines.push(self.lines.get(line_number - 1));
        }
        target_lines.into_iter().flatten().collect_vec()
    }

    fn get_target_gear_lines(&self, line_number: usize) -> Vec<&SchematicLine> {
        let target_lines = vec![self.lines.get(line_number), self.lines.get(line_number + 1)];
        target_lines.into_iter().flatten().collect_vec()
    }
    fn gears(&self) -> Result<Vec<u32>> {
        let mut parts = Vec::new();
        for (idx, line) in self.lines.iter().enumerate() {
            for (pos, s) in line.part_numbers.iter() {
                let start = if *pos == 0 { 0 } else { *pos - 1 };
                let end = pos + s.len();
                let positions = start..=end;
                let target_lines = self.get_target_lines(idx);
                if target_lines.iter().any(|l| l.is_symbol(positions.clone())) {
                    // we found a part, now does it touch a gear?
                    // get the set of lines that this part number touches a gear for
                    let gear_lines: HashMap<&SchematicLine, Vec<usize>> = target_lines
                        .into_iter()
                        .map(|l| (l, l.touches_gear(positions.clone())))
                        .filter(|l| !l.1.is_empty())
                        .collect();
                    let mut part_numbers: HashMap<usize, Vec<String>> = HashMap::new();
                    for (l, gear_positions) in gear_lines {
                        for gt in self.get_target_gear_lines(l.line_number) {
                            if part_numbers.contains_key(&gt.line_number) {
                                continue;
                            }
                            println!(
                                "gear positions {} - {}",
                                gt.line_number,
                                gear_positions.iter().join(",")
                            );
                            for g_idx in gear_positions.clone() {
                                let gear = gt.touches_part_numbers(g_idx);
                                if !gear.is_empty() {
                                    println!(
                                        "{} - {} - {} - {}",
                                        l.line_number,
                                        gt.line_number,
                                        g_idx,
                                        gear.iter().join(",")
                                    );
                                    part_numbers.insert(gt.line_number, gear);
                                }
                            }
                        }
                    }
                    let p = part_numbers.values().flatten().collect_vec();
                    if p.len() == 2 {
                        //println!("{}", p.iter().join(","));
                        let n = p[0].parse::<u32>()?;
                        let m = p[1].parse::<u32>()?;
                        parts.push(n * m);
                    }
                }
            }
        }
        Ok(parts)
    }
}

fn solve_part1() -> Result<u32> {
    let data = include_str!("input.txt");
    let s = Schematic::new(data);
    let parts = s.part_numbers()?;
    Ok(parts.iter().sum())
}

fn solve_part2() -> Result<u32> {
    let data = include_str!("input.txt");
    let s = Schematic::new(data);
    let parts = s.gears()?;
    println!("{}", parts.iter().join(","));
    Ok(parts.iter().sum())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn day3_part1() -> Result<()> {
        let s = solve_part1()?;
        println!("{}", s);
        assert_eq!(s, 532331);
        Ok(())
    }

    #[test]
    fn day3_part2() -> Result<()> {
        let s = solve_part2()?;
        println!("{}", s);
        assert_eq!(s, 82301120);
        Ok(())
    }
}
