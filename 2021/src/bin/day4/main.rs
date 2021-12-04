const INPUT_FILE_PATH: &str = "src/bin/day4/input";

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
    let lines = lib::lines_from_file(INPUT_FILE_PATH).expect("Could not load input data");
    println!("Part 1");
    part1(lines.clone());
    println!("Part 2");
    part2(lines.clone());
}

fn part1(lines: Vec<String>) {
    let numbers: Vec<String> = lines[0].split(",").map(String::from).collect();

    let mut boards: Vec<Vec<Vec<Cell>>> = Vec::new();
    boards.push(Vec::new());
    let mut current_board = 0;
    for i in 2..lines.len() {
        let line = String::from(&lines[i]);

        if line == "" {
            current_board += 1;
            boards.push(Vec::new());
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
        boards[current_board].push(column_vector);
    }

    for round in 0..numbers.len() {
        let bingo_number: String = numbers[round].clone();

        for board_i in 0..boards.len() {
            if update_and_check_board_1(&mut boards[board_i], bingo_number.clone()) {
                let mut unmatched_sum: u32 = 0;
                for row_i in 0..boards[board_i].len() {
                    let row = &boards[board_i][row_i];
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
                return;
            }
        }
    }
}

fn part2(lines: Vec<String>) {
    let numbers: Vec<String> = lines[0].split(",").map(String::from).collect();

    let mut boards: Vec<Game> = Vec::new();
    boards.push(Game {
        rows: Vec::new(),
        complete: false,
    });
    let mut current_board = 0;
    for i in 2..lines.len() {
        let line = String::from(&lines[i]);

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
            if update_and_check_board_2(&mut boards[board_i], bingo_number.clone()) {
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

fn update_and_check_board_1(game: &mut Vec<Vec<Cell>>, bingo_number: String) -> bool {
    for row_i in 0..game.len() {
        let row = &mut game[row_i];
        for col_i in 0..row.len() {
            let col = &mut row[col_i];
            if col.value == bingo_number {
                col.matched = true;
            }
        }
    }

    // Check if any rows match
    for row_i in 0..game.len() {
        let row = &mut game[row_i];
        let mut row_is_all_matched = true;
        for col_i in 0..row.len() {
            let col = &mut row[col_i];
            if !col.matched {
                row_is_all_matched = false;
                break;
            }
        }
        if row_is_all_matched {
            return true;
        }
    }

    // check if any columns match
    for col_i in 0..game[0].len() {
        let mut col_is_all_matched = true;

        for row_i in 0..game.len() {
            let col = &mut game[row_i][col_i];
            if !col.matched {
                col_is_all_matched = false;
                break;
            }
        }

        if col_is_all_matched {
            return true;
        }
    }

    return false;
}

fn update_and_check_board_2(game: &mut Game, bingo_number: String) -> bool {
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
