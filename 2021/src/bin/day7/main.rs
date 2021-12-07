const INPUT_FILE_PATH: &str = "src/bin/day7/input";

fn main() {
    let lines = lib::lines_from_file(INPUT_FILE_PATH).expect("Could not load input data");
    println!("Part 1");
    part1(&lines);
    println!("Part 2");
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let mut positions: Vec<u32> = lines[0]
        .split(",")
        .map(|pos| pos.parse::<u32>().unwrap())
        .collect();
    positions.sort();

    let median: u32 = positions[positions.len() / 2];

    let mut fuel_used: u32 = 0;
    for pos in positions.iter() {
        fuel_used += (*pos as i32 - median as i32).abs() as u32;
    }

    println!("{}", fuel_used);
}

fn part2(lines: &Vec<String>) {
    let positions: Vec<u32> = lines[0]
        .split(",")
        .map(|pos| pos.parse::<u32>().unwrap())
        .collect();

    let mean: u32 = positions.iter().sum::<u32>() / positions.len() as u32;

    let mut fuel_used: u32 = 0;
    for pos in positions.iter() {
        let diff: u32 = (*pos as i32 - mean as i32).abs() as u32;
        fuel_used += (diff.pow(2) + diff) / 2;
    }

    println!("{}", fuel_used);
}
