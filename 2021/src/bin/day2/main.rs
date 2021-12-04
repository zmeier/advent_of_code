const INPUT_FILE_PATH: &str = "src/bin/day2/input";

fn main() {
    let lines = lib::lines_from_file(INPUT_FILE_PATH).expect("Could not load input data");
    println!("Part 1");
    part1(lines.clone());
    println!("Part 2");
    part2(lines.clone());
}

fn part1(lines: Vec<String>) {
    let mut horizontal = 0;
    let mut depth = 0;

    for line in lines {
        if line.starts_with("forward ") {
            horizontal += &line["forward ".len()..].parse::<i32>().unwrap()
        } else if line.starts_with("down ") {
            depth += &line["down ".len()..].parse::<i32>().unwrap()
        } else if line.starts_with("up ") {
            depth -= &line["up ".len()..].parse::<i32>().unwrap()
        }
    }

    println!("Total depth changes: [{}]", horizontal * depth);
}

fn part2(lines: Vec<String>) {
    let mut aim = 0;
    let mut horizontal = 0;
    let mut depth = 0;

    for line in lines {
        let mut pieces = line.split_whitespace();
        let instruction = pieces.next();
        let units = pieces
            .next()
            .map(|s| s.parse::<i32>().unwrap())
            .unwrap_or(0);
        match instruction {
            Some("forward") => {
                horizontal += units;
                depth += aim * units;
            }
            Some("down") => aim += units,
            Some("up") => aim -= units,
            _ => println!("Bad instruction [{:?}]", instruction),
        }
    }

    println!("Total depth changes: [{}]", horizontal * depth);
}
