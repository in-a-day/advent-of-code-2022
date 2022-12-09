use std::io::{stdin, Read};

fn main() {
    let mut input = stdin();
    let mut cnt = String::new();
    if input.read_to_string(&mut cnt).is_ok() {
        println!("part1: {:?}", part1(&cnt));
        println!("part2: {:?}", part2(&cnt));
    }
}

fn part1(input: &str) -> Option<usize> {
    find_marker(input, 4)
}

fn part2(input: &str) -> Option<usize> {
    find_marker(input, 14)
}

fn find_marker(input: &str, win_size: usize) -> Option<usize> {
    let mut win: Vec<u8> = Vec::with_capacity(win_size);
    for (i, v) in input.bytes().enumerate() {
        if win.contains(&v) {
            for _ in 0..win.len() {
                if win.remove(0) == v {
                    break;
                }
            }
        }
        win.push(v);
        if win.len() == win_size {
            return Some(i + 1);
        }
    }

    None
}
