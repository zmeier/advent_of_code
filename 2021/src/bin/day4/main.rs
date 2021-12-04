const INPUT_FILE_PATH: &str = "src/bin/day4/input";

#[derive(Debug)]
struct Board {
    rows: Vec<Vec<Cell>>,
    complete: bool,
}

#[derive(Debug)]
struct Cell {
    value: u32,
    matched: bool,
}

fn main() {
    let lines = lib::lines_from_file(INPUT_FILE_PATH).expect("Could not load input data");
    println!("Part 1");
    part1(&lines);
    println!("Part 2");
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let (bingo_numbers, mut boards) = get_game_data(&lines);

    for bingo_number in bingo_numbers {
        for board in boards.iter_mut() {
            update_with_bingo_number(board, bingo_number);
            if is_board_a_winner(&board) {
                board.complete = true;
                let unmatched_sum = sum_unmatched_cells(&board);
                println!(
                    "{}*{}={}",
                    bingo_number,
                    unmatched_sum,
                    bingo_number * unmatched_sum
                );
                return;
            }
        }
    }
}

fn part2(lines: &Vec<String>) {
    let (bingo_numbers, mut boards) = get_game_data(&lines);

    let mut last_winning_bingo_number: u32 = bingo_numbers[0];
    let mut last_umatched_sum: u32 = 0;

    for bingo_number in bingo_numbers {
        for board in boards.iter_mut() {
            if board.complete {
                continue;
            }

            update_with_bingo_number(board, bingo_number);
            if is_board_a_winner(&board) {
                board.complete = true;
                last_winning_bingo_number = bingo_number;
                last_umatched_sum = sum_unmatched_cells(&board);
            }
        }
    }

    println!(
        "{}*{}={}",
        last_winning_bingo_number,
        last_umatched_sum,
        last_winning_bingo_number * last_umatched_sum
    );
}

fn get_game_data(lines: &Vec<String>) -> (Vec<u32>, Vec<Board>) {
    let bingo_numbers: Vec<u32> = lines[0]
        .split(",")
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    let mut boards: Vec<Board> = Vec::new();
    boards.push(Board {
        rows: Vec::new(),
        complete: false,
    });
    let mut current_board_idx = 0;
    for i in 2..lines.len() {
        let line = &lines[i];

        if line.is_empty() {
            current_board_idx += 1;
            boards.push(Board {
                rows: Vec::new(),
                complete: false,
            });
            continue;
        }

        let mut cells: Vec<Cell> = Vec::new();
        let line_pieces = line.split_whitespace();
        for piece in line_pieces {
            let cell: Cell = Cell {
                value: piece.parse::<u32>().unwrap(),
                matched: false,
            };
            cells.push(cell);
        }
        boards[current_board_idx].rows.push(cells);
    }

    (bingo_numbers, boards)
}

fn update_with_bingo_number(board: &mut Board, bingo_number: u32) {
    for row in board.rows.iter_mut() {
        for col in row.iter_mut() {
            if col.value == bingo_number {
                col.matched = true;
            }
        }
    }
}

fn is_board_a_winner(board: &Board) -> bool {
    // Check if any rows match
    for row in board.rows.iter() {
        let mut row_is_all_matched = true;
        for col in row.iter() {
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
    for col_i in 0..board.rows[0].len() {
        let mut col_is_all_matched = true;
        for row_i in 0..board.rows.len() {
            if !board.rows[row_i][col_i].matched {
                col_is_all_matched = false;
                break;
            }
        }
        if col_is_all_matched {
            return true;
        }
    }

    false
}

fn sum_unmatched_cells(board: &Board) -> u32 {
    let mut unmatched_sum: u32 = 0;
    for row in board.rows.iter() {
        for col in row.iter() {
            if !col.matched {
                unmatched_sum += col.value;
            }
        }
    }
    unmatched_sum
}
