use std::cmp::Ordering;
use std::collections::BinaryHeap;

const INPUT_FILE_PATH: &str = "src/bin/day15/input";
const MOVE_MASK: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

// Details used to implement the Priority Queue for the cavern state
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u128,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let lines = lib::lines_from_file(INPUT_FILE_PATH).expect("Could not load input data");
    println!("Part 1");
    part1(&lines);
    println!("Part 2");
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let cavern = get_initial_cavern(lines);
    let lowest_cost = find_shortest_path(&cavern);
    println!(
        "{}",
        lowest_cost.expect("Something went wrong and did not find path")
    );
}

fn part2(lines: &Vec<String>) {
    let initial_cavern = get_initial_cavern(lines);
    let initial_cavern_size = initial_cavern.len();
    let cavern_multiplier = 5;
    let cavern_size: usize = initial_cavern_size * cavern_multiplier;
    let mut cavern = vec![vec![u128::MAX; cavern_size]; cavern_size];

    // Populate initial cavern
    for row in 0..initial_cavern_size {
        for col in 0..initial_cavern_size {
            cavern[row][col] = initial_cavern[row][col];
        }
    }

    // Copy and increase value downward
    for row in initial_cavern_size..cavern_size {
        for col in 0..initial_cavern_size {
            cavern[row][col] =
                get_incremented_value_with_cap(cavern[row - initial_cavern_size][col]);
        }
    }

    // Copy and increase value across
    for row in 0..cavern_size {
        for col in initial_cavern_size..cavern_size {
            cavern[row][col] =
                get_incremented_value_with_cap(cavern[row][col - initial_cavern_size]);
        }
    }

    let lowest_cost = find_shortest_path(&cavern);
    println!(
        "{}",
        lowest_cost.expect("Something went wrong and did not find path")
    );
}

fn get_incremented_value_with_cap(val: u128) -> u128 {
    if val == 9 {
        return 1;
    }
    val + 1
}

fn get_initial_cavern(lines: &Vec<String>) -> Vec<Vec<u128>> {
    let mut cavern: Vec<Vec<u128>> = Vec::new();

    for (row, line) in lines.iter().enumerate() {
        cavern.push(Vec::new());
        for val in line.chars() {
            let cell_val: u128 = val.to_digit(10).unwrap() as u128;
            cavern[row].push(cell_val);
        }
    }

    cavern
}

fn find_shortest_path(cavern: &Vec<Vec<u128>>) -> Option<u128> {
    let cavern_size: usize = cavern.len();
    let mut costs: Vec<Vec<u128>> = vec![vec![u128::MAX; cavern_size]; cavern_size];
    costs[0][0] = cavern[0][0];

    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    heap.push(State {
        cost: cavern[0][0],
        position: (0, 0),
    });

    while let Some(State { cost, position }) = heap.pop() {
        let (row, col) = position;
        if position == (cavern_size - 1, cavern_size - 1) {
            return Some(cost - cavern[0][0]);
        }

        if cost > costs[row][col] {
            continue;
        }

        for mask in MOVE_MASK {
            let (move_row, move_col) = mask;
            let new_row: isize = row as isize + move_row;
            let new_col: isize = col as isize + move_col;
            // Negative numbers cannot be created because it will go outside the bounds of usize data type
            if new_row >= 0
                && new_col >= 0
                && new_row < cavern_size as isize
                && new_col < cavern_size as isize
            {
                let next = State {
                    cost: cost + cavern[new_row as usize][new_col as usize],
                    position: (new_row as usize, new_col as usize),
                };
                if next.cost < costs[new_row as usize][new_col as usize] {
                    heap.push(next);
                    costs[new_row as usize][new_col as usize] = next.cost;
                }
            }
        }
    }

    None
}
