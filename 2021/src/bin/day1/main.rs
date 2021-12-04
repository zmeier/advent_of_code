const INPUT_FILE_PATH: &str = "src/bin/day1/input";

fn main() {
    let lines = lib::lines_from_file(INPUT_FILE_PATH).expect("Could not load input data");
    println!("Part 1");
    part1(&lines);
    println!("Part 2");
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let mut output = 0;

    for depth_index in 1..lines.len() {
        let previous_depth = &lines[depth_index - 1].parse::<i32>().unwrap();
        let depth = &lines[depth_index].parse::<i32>().unwrap();
        if previous_depth < depth {
            output += 1;
        }
    }

    println!("{}", output);
}

fn part2(lines: &Vec<String>) {
    let mut merged_depths = Vec::new();
    
    for depth_index in 0..lines.len() {
        let depth = lines[depth_index].parse::<i32>().unwrap();
        merged_depths.push(depth);
        for i in 1..3 {
            if (depth_index as i32) - (i as i32) >= 0 {
                merged_depths[depth_index - i] += depth;
            }
        }
    }

    let mut output = 0;

    for depth_index in 1..merged_depths.len() {
        let previous_depth = &merged_depths[depth_index - 1];
        let depth = &merged_depths[depth_index];

        if previous_depth < depth {
            output += 1;
        }
    }

    println!("{}", output);
}
