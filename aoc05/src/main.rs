use regex::Regex;
use std::{
    io::{stdin, Read},
    str::FromStr,
};

#[macro_use]
extern crate lazy_static;

type Stack = Vec<String>;

fn main() {
    let mut input = stdin();
    let mut cnt = String::new();
    input.read_to_string(&mut cnt).unwrap();
    let mc: Machine = cnt.parse().unwrap();

    println!("part1: {}", part1(&mc));
    println!("part1: {}", part2(&mc));
}

fn part1(mc: &Machine) -> String {
    mc.move_one()
}

fn part2(mc: &Machine) -> String {
    mc.move_all()
}

#[derive(Debug, Clone)]
struct Machine {
    stacks: Vec<Stack>,
    commands: Vec<Command>,
}

impl Machine {
    fn new(stacks: Vec<Stack>, commands: Vec<Command>) -> Self {
        Self { stacks, commands }
    }

    // move one crate per step
    fn move_one(&self) -> String {
        let mut stacks = self.stacks.clone();
        for cmd in &self.commands {
            for _ in 0..cmd.crates {
                if let Some(v) = stacks[(cmd.source - 1) as usize].pop() {
                    stacks[(cmd.target - 1) as usize].push(v);
                }
            }
        }

        Self::top(stacks)
    }

    // move all crates per step
    fn move_all(&self) -> String {
        let mut stacks = self.stacks.clone();
        for cmd in &self.commands {
            let mut cache = vec![];
            for _ in 0..cmd.crates {
                if let Some(v) = stacks[(cmd.source - 1) as usize].pop() {
                    cache.insert(0, v);
                }
            }
            stacks[(cmd.target - 1) as usize].extend(cache);
        }

        Self::top(stacks)
    }

    // top char of each stack
    fn top(stacks: Vec<Stack>) -> String {
        stacks
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| s.last().unwrap().to_owned())
            .collect()
    }

    fn collect_cmds(input: &str) -> Vec<Command> {
        input
            .lines()
            .map(|l| l.parse::<Command>().unwrap())
            .collect()
    }

    fn collect_stacks(input: &str) -> Vec<Stack> {
        let mut stacks = vec![];

        for line in input.lines().rev().skip(1) {
            line.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|&(_, c)| c != ' ')
                .for_each(|(i, c)| {
                    if i == stacks.len() {
                        stacks.push(vec![]);
                    }
                    stacks[i].push(c.to_string());
                });
        }

        stacks
    }
}

impl FromStr for Machine {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut is = s.split("\n\n");
        let stacks_str = is.next().unwrap();
        let cmds_str = is.next().unwrap();

        let stacks = Self::collect_stacks(stacks_str);
        let cmds = Self::collect_cmds(cmds_str);

        Ok(Self::new(stacks, cmds))
    }
}

#[derive(Debug, Clone, Copy)]
struct Command {
    crates: u16,
    source: u8,
    target: u8,
}

impl FromStr for Command {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // move 1 from 2 to 1
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
                move\s
                (?P<crates>\d+)\s
                from\s
                (?P<source>\d+)\s
                to\s
                (?P<target>\d+)
                "
            )
            .unwrap();
        }

        let caps = RE.captures(s).unwrap();

        Ok(Self {
            crates: caps["crates"].parse()?,
            source: caps["source"].parse()?,
            target: caps["target"].parse()?,
        })
    }
}
