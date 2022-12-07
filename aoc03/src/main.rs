use std::io::{stdin, Read};

fn main() {
    let mut input = stdin();
    let mut content = String::new();

    if input.read_to_string(&mut content).is_ok() {
        println!("part1: {}", part1(&content));
        println!("part2: {}", part2(&content));
    }
}

fn part1(input: &str) -> usize {
    let mut ret: usize = 0;
    for line in input.lines() {
        let mut items = [0; 52];
        for (count, val) in line.bytes().enumerate() {
            let idx = to_index(val);
            if count < line.len() / 2 {
                items[idx] = (items[idx] + 1).min(1);
            } else if items[idx] == 1 {
                items[idx] += 1;
                ret += to_val(idx);
            }
        }
    }

    ret
}

fn part2(input: &str) -> usize {
    let mut ret = 0;
    let mut count = 1;
    let mut items = [0; 52];
    for line in input.lines() {
        'outer: for b in line.bytes() {
            let idx = to_index(b);
            match count {
                1 => items[idx] = 1,
                2 if items[idx] == 1 => items[idx] = 2,
                3 if items[idx] == 2 => {
                    ret += to_val(idx);
                    break 'outer;
                }
                _ => (),
            }
        }

        count += 1;
        if count == 4 {
            count = 1;
            items = [0; 52];
        }
    }

    ret
}

fn to_index(val: u8) -> usize {
    if val > 96 {
        val as usize - 97
    } else {
        val as usize - 65 + 26
    }
}

fn to_val(idx: usize) -> usize {
    idx + 1
}
