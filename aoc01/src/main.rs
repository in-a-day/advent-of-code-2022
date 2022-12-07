use std::io::{stdin, Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = stdin();
    let mut content = String::new();
    input.read_to_string(&mut content)?;

    let mut calories = vec![];
    let mut curr = 0;
    for line in content.lines() {
        if line.is_empty() {
            calories.push(curr);
            curr = 0;
        } else {
            curr += line.parse::<u32>()?;
        }
    }

    println!("part1: {}", part1(&calories));
    println!("part2: {}", part2(&calories));

    Ok(())
}

fn part1(calories: &Vec<u32>) -> u32 {
    *calories.iter().max().unwrap_or(&0)
}

fn part2(calories: &Vec<u32>) -> u32 {
    let mut order = (0, 0, 0);
    for &calory in calories {
        if calory >= order.0 {
            order = (calory, order.0, order.1);
        } else if calory >= order.1 {
            order = (order.0, calory, order.1);
        } else if calory >= order.2 {
            order = (order.0, order.1, calory);
        }
    }

    order.0 + order.1 + order.2
}
