use std::{
    io::{stdin, Read},
    str::FromStr,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    let mut input = stdin();
    let mut content = String::new();

    if input.read_to_string(&mut content).is_ok() {
        let records: Vec<_> = content
            .lines()
            .map(|line| line.parse::<Record>().unwrap())
            .collect();
        println!("{:?}", part1(&records));
        println!("{:?}", part2(&records));
    }
}

fn part1(records: &Vec<Record>) -> Result<u32> {
    let mut score = 0;
    for record in records {
        let left = record.left.parse::<Sharp>()?;
        let right = if record.right == "X" {
            Ok(Sharp::A)
        } else if record.right == "Y" {
            Ok(Sharp::B)
        } else if record.right == "Z" {
            Ok(Sharp::C)
        } else {
            Err("unknown sharp")
        }?;
        score += right.score() + right.compare_to(&left).score();
    }

    Ok(score)
}

fn part2(records: &Vec<Record>) -> Result<u32> {
    let mut score = 0;
    for record in records {
        let left = record.left.parse::<Sharp>()?;
        let rt = record.right.parse::<ResultType>()?;
        score += rt.score() + left.response(&rt).score();
    }

    Ok(score)
}

struct Record {
    left: String,
    right: String,
}

impl FromStr for Record {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let x: Vec<&str> = s.split(' ').collect();
        Ok(Self {
            left: x[0].to_owned(),
            right: x[1].to_owned(),
        })
    }
}

#[allow(dead_code)]
enum ResultType {
    Lost = 0,
    Draw = 3,
    Win = 6,
}

impl ResultType {
    fn score(&self) -> u32 {
        *self as u32
    }
}

impl FromStr for ResultType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "X" => Ok(ResultType::Lost),
            "Y" => Ok(ResultType::Draw),
            "Z" => Ok(ResultType::Win),
            _ => Err("parse result type error".to_owned()),
        }
    }
}

#[derive(PartialEq, Eq)]
enum Sharp {
    A = 1,
    B = 2,
    C = 3,
}

impl Sharp {
    // compare to result, self win
    fn win(&self) -> Self {
        use Sharp::*;
        match self {
            A => C,
            B => A,
            C => B,
        }
    }

    fn draw(&self) -> Self {
        use Sharp::*;
        match self {
            A => A,
            B => B,
            C => C,
        }
    }

    // compare to other, self lost
    fn lost(&self) -> Self {
        use Sharp::*;
        match self {
            A => B,
            B => C,
            C => A,
        }
    }

    fn score(&self) -> u32 {
        *self as u32
    }

    fn compare_to(&self, other: &Sharp) -> ResultType {
        if self.win() == *other {
            ResultType::Win
        } else if self.draw() == *other {
            ResultType::Draw
        } else {
            ResultType::Lost
        }
    }

    // rt is result of using other compare to self
    fn response(&self, rt: &ResultType) -> Self {
        match rt {
            ResultType::Win => self.lost(),
            ResultType::Draw => self.draw(),
            ResultType::Lost => self.win(),
        }
    }
}

impl FromStr for Sharp {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "A" => Ok(Sharp::A),
            "B" => Ok(Sharp::B),
            "C" => Ok(Sharp::C),
            _ => Err("parse sharp error".to_owned()),
        }
    }
}
