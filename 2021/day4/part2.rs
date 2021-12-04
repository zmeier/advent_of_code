use std::fs::File;
use std::io::{self, BufRead, BufReader};

const INPUT_FILE_NAME: &str = "./input.txt";

#[derive(Debug)]
struct Game {
    rows: Vec<Vec<Cell>>,
    complete: bool,
}

#[derive(Debug)]
struct Cell {
    value: String,
    matched: bool,
}

fn main() {
    let file_lines = load_file_data().expect("Could not load lines");

    let numbers: Vec<String> = file_lines[0].split(",").map(String::from).collect();

    let mut boards: Vec<Game> = Vec::new();
    boards.push(Game {
        rows: Vec::new(),
        complete: false,
    });
    let mut current_board = 0;
    for i in 2..file_lines.len() {
        let line = String::from(&file_lines[i]);

        if line == "" {
            current_board += 1;
            boards.push(Game {
                rows: Vec::new(),
                complete: false,
            });
            continue;
        }

        let mut column_vector: Vec<Cell> = Vec::new();
        let line_pieces = line.split_whitespace();
        for piece in line_pieces {
            let cell: Cell = Cell {
                value: String::from(piece),
                matched: false,
            };
            column_vector.push(cell);
        }
        boards[current_board].rows.push(column_vector);
    }

    for round in 0..numbers.len() {
        let bingo_number: String = numbers[round].clone();

        for board_i in 0..boards.len() {
            if update_and_check_board(&mut boards[board_i], bingo_number.clone()) {
                let mut unmatched_sum: u32 = 0;
                for row_i in 0..boards[board_i].rows.len() {
                    let row = &boards[board_i].rows[row_i];
                    for col_i in 0..row.len() {
                        let col = &row[col_i];
                        if !col.matched {
                            unmatched_sum += col.value.parse::<u32>().unwrap();
                        }
                    }
                }
                println!(
                    "{}*{}={}",
                    unmatched_sum,
                    bingo_number,
                    unmatched_sum * bingo_number.parse::<u32>().unwrap()
                );
            }
        }
    }
}

fn update_and_check_board(game: &mut Game, bingo_number: String) -> bool {
    if game.complete {
        return false;
    }

    for row_i in 0..game.rows.len() {
        let row = &mut game.rows[row_i];
        for col_i in 0..row.len() {
            let col = &mut row[col_i];
            if col.value == bingo_number {
                col.matched = true;
            }
        }
    }

    // Check if any rows match
    for row_i in 0..game.rows.len() {
        let row = &mut game.rows[row_i];
        let mut row_is_all_matched = true;
        for col_i in 0..row.len() {
            let col = &mut row[col_i];
            if !col.matched {
                row_is_all_matched = false;
                break;
            }
        }
        if row_is_all_matched {
            game.complete = true;
            return true;
        }
    }

    // check if any columns match
    for col_i in 0..game.rows[0].len() {
        let mut col_is_all_matched = true;

        for row_i in 0..game.rows.len() {
            let col = &mut game.rows[row_i][col_i];
            if !col.matched {
                col_is_all_matched = false;
                break;
            }
        }

        if col_is_all_matched {
            game.complete = true;
            return true;
        }
    }

    return false;
}

fn load_file_data() -> io::Result<Vec<String>> {
    BufReader::new(File::open(INPUT_FILE_NAME)?)
        .lines()
        .collect()
}
