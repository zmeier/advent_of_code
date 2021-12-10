use std::collections::HashMap;

const INPUT_FILE_PATH: &str = "src/bin/day10/input";

fn main() {
    let lines = lib::lines_from_file(INPUT_FILE_PATH).expect("Could not load input data");
    println!("Part 1");
    part1(&lines);
    println!("Part 2");
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let illegal_char_scores: HashMap<char, u32> =
        HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let start_to_close_char: HashMap<char, char> =
        HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

    let mut score: u32 = 0;
    for line in lines.iter() {
        let mut stack: Vec<char> = Vec::new();
        for c in line.chars() {
            if start_to_close_char.contains_key(&c) {
                stack.push(c);
            } else {
                let closing: char = stack.pop().unwrap();
                if c != *start_to_close_char.get(&closing).unwrap() {
                    score += illegal_char_scores.get(&c).unwrap();
                }
            }
        }
    }
    println!("{}", score);
}

fn part2(lines: &Vec<String>) {
    let finishing_char_scores: HashMap<char, u32> =
        HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    let start_to_close_char: HashMap<char, char> =
        HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

    let mut all_scores: Vec<u128> = Vec::new();

    for line in lines.iter() {
        let mut stack: Vec<char> = Vec::new();
        let mut invalid = false;
        for c in line.chars() {
            if start_to_close_char.contains_key(&c) {
                stack.push(c);
            } else {
                let starting: char = stack.pop().unwrap();
                if c != *start_to_close_char.get(&starting).unwrap() {
                    // invalid line so break out
                    invalid = true;
                    break;
                }
            }
        }

        if invalid || stack.len() == 0 {
            continue;
        }

        let mut score: u128 = 0;
        while let Some(c) = stack.pop() {
            let closing = *start_to_close_char.get(&c).unwrap();
            score = score * 5 + *finishing_char_scores.get(&closing).unwrap() as u128;
        }

        all_scores.push(score);
    }

    all_scores.sort();
    println!("{}", all_scores[all_scores.len() / 2]);
}
