use std::collections::HashMap;

const INPUT_FILE_PATH: &str = "src/bin/day14/input";

fn main() {
    let lines = lib::lines_from_file(INPUT_FILE_PATH).expect("Could not load input data");
    println!("Part 1");
    part1(&lines);
    println!("Part 2");
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let (mut pieces, insertions) = get_sequence_and_insertions(lines);

    for _ in 0..10 {
        pieces = track_patterns(&pieces, &insertions);
    }

    let (highest_count, lowest_count) = find_highest_and_lowest(pieces);

    println!("{}", highest_count - lowest_count);
}

fn part2(lines: &Vec<String>) {
    let (mut pieces, insertions) = get_sequence_and_insertions(lines);

    for _ in 0..40 {
        pieces = track_patterns(&pieces, &insertions);
    }

    let (highest_count, lowest_count) = find_highest_and_lowest(pieces);

    println!("{}", highest_count - lowest_count);
}

fn get_sequence_and_insertions(
    lines: &Vec<String>,
) -> (HashMap<(char, char), u128>, HashMap<(char, char), char>) {
    let mut line_iterator = lines.iter();

    let sequence: Vec<char> = line_iterator.next().unwrap().chars().collect();
    let mut pieces: HashMap<(char, char), u128> = HashMap::new();
    for i in 0..sequence.len() - 1 {
        let combo = (sequence[i], sequence[i + 1]);
        let char_count: u128 = pieces.get(&combo).unwrap_or(&0) + 1;
        pieces.insert(combo, char_count);
    }
    pieces.insert((sequence[sequence.len() - 1], '_'), 1);

    line_iterator.next();

    let mut insertions: HashMap<(char, char), char> = HashMap::new();
    for line in line_iterator {
        let pieces: Vec<&str> = line.split(" -> ").collect();
        let keys: Vec<char> = pieces[0].chars().collect();
        insertions.insert((keys[0], keys[1]), pieces[1].chars().next().unwrap());
    }

    (pieces, insertions)
}

fn track_patterns(
    pieces: &HashMap<(char, char), u128>,
    insertions: &HashMap<(char, char), char>,
) -> HashMap<(char, char), u128> {
    let mut new_pieces: HashMap<(char, char), u128> = HashMap::new();
    for (piece_tuple, val) in pieces.clone() {
        let (piece_1, piece_2) = piece_tuple;
        if piece_2 == '_' {
            // special piece that is used to track last element of sequence
            new_pieces.insert(piece_tuple, val);
            continue;
        }

        if let Some(insertion) = insertions.get(&piece_tuple) {
            let new_piece_tuple_1 = (piece_1, *insertion);
            let char_count_1: u128 = new_pieces.get(&new_piece_tuple_1).unwrap_or(&0) + val;
            new_pieces.insert(new_piece_tuple_1, char_count_1);

            let new_piece_tuple_2 = (*insertion, piece_2);
            let char_count_2: u128 = new_pieces.get(&new_piece_tuple_2).unwrap_or(&0) + val;
            new_pieces.insert(new_piece_tuple_2, char_count_2);
        }
    }

    new_pieces
}

fn find_highest_and_lowest(pieces: HashMap<(char, char), u128>) -> (u128, u128) {
    let mut counts: HashMap<char, u128> = HashMap::new();
    for (key, val) in pieces.iter() {
        let (c, _) = key;
        let char_count: u128 = *counts.get(c).unwrap_or(&0) + val;
        counts.insert(*c, char_count);
    }

    let mut lowest_count: u128 = u128::MAX;
    let mut highest_count: u128 = u128::MIN;
    for (_, val) in counts.iter() {
        if *val < lowest_count {
            lowest_count = *val;
        }
        if *val > highest_count {
            highest_count = *val;
        }
    }

    (highest_count, lowest_count)
}
