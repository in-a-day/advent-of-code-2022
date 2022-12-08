use std::{
    io::{stdin, Read},
    str::FromStr,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = stdin();
    let mut cnt = String::new();
    input.read_to_string(&mut cnt)?;

    let records: Vec<_> = cnt
        .lines()
        .map(|l| l.split(',').map(|s| s.parse::<Range>().unwrap()))
        .map(|mut s| (s.next().unwrap(), s.next().unwrap()))
        .collect();

    println!("part1: {}", part1(&records));
    println!("part2: {}", part2(&records));

    Ok(())
}

fn part1(records: &[(Range, Range)]) -> usize {
    records
        .iter()
        .filter(|(a, b)| a.fully_contains(b) || b.fully_contains(a))
        .count()
}

fn part2(records: &[(Range, Range)]) -> usize {
    records.iter().filter(|(a, b)| a.overlap(b)).count()
}

#[derive(Debug, Clone, Copy)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn fully_contains(&self, other: &Self) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    fn overlap(&self, other: &Self) -> bool {
        !(self.end < other.start || other.end < self.start)
    }
}

impl FromStr for Range {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let x: Vec<_> = s.split('-').collect();
        Ok(Self {
            start: x[0].parse()?,
            end: x[1].parse()?,
        })
    }
}
