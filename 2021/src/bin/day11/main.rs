const INPUT_FILE_PATH: &str = "src/bin/day11/input";
const MOVE_MASK: [(isize, isize); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (1, 1),
    (-1, 1),
    (1, -1),
];

fn main() {
    let lines = lib::lines_from_file(INPUT_FILE_PATH).expect("Could not load input data");
    println!("Part 1");
    part1(&lines);
    println!("Part 2");
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let mut grid: Vec<Vec<u32>> = build_grid(&lines);

    let mut num_pulses = 0;
    for _ in 1..=100 {
        num_pulses += pulse(&mut grid);
    }

    println!("{}", num_pulses);
}

fn part2(lines: &Vec<String>) {
    let mut grid: Vec<Vec<u32>> = build_grid(&lines);

    let mut i = 1;
    while pulse(&mut grid) != 100 {
        i += 1;
    }
    println!("{}", i);
}

fn build_grid(lines: &Vec<String>) -> Vec<Vec<u32>> {
    let mut grid: Vec<Vec<u32>> = Vec::new();
    for line in lines.iter() {
        let row: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        grid.push(row);
    }
    grid
}

fn pulse(grid: &mut Vec<Vec<u32>>) -> u32 {
    let mut pulses: Vec<(usize, usize)> = Vec::new();

    let mut num_pulses = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 9 {
                grid[row][col] = 0;
                pulses.push((row, col));
                num_pulses += 1;
            } else {
                grid[row][col] += 1;
            }
        }
    }

    while let Some((row, col)) = pulses.pop() {
        for mask in MOVE_MASK.iter() {
            let (move_row, move_col) = mask;
            let new_row: isize = row as isize + move_row;
            let new_col: isize = col as isize + move_col;
            // Negative numbers cannot be created because it will go outside the bounds of usize data type
            if new_row >= 0
                && new_col >= 0
                && new_row < grid.len() as isize
                && new_col < grid[new_row as usize].len() as isize
                && grid[new_row as usize][new_col as usize] != 0
            {
                if grid[new_row as usize][new_col as usize] == 9 {
                    grid[new_row as usize][new_col as usize] = 0;
                    num_pulses += 1;
                    pulses.push((new_row as usize, new_col as usize));
                } else {
                    grid[new_row as usize][new_col as usize] += 1;
                }
            }
        }
    }

    num_pulses
}
