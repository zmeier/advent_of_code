const INPUT_FILE_PATH: &str = "src/bin/day6/input";

fn main() {
    let lines = lib::lines_from_file(INPUT_FILE_PATH).expect("Could not load input data");
    println!("Part 1");
    part1(&lines);
    println!("Part 2");
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let mut ages: Vec<u32> = lines[0]
        .split(",")
        .map(|age| age.parse::<u32>().unwrap())
        .collect();

    for _ in 1..=80 {
        let mut mut_new_ages: Vec<u32> = Vec::new();
        for age in ages {
            if age == 0 {
                mut_new_ages.push(8);
                mut_new_ages.push(6);
            } else {
                mut_new_ages.push(age - 1);
            }
        }

        ages = mut_new_ages;
    }

    println!("{}", ages.len());
}
fn part2(lines: &Vec<String>) {
    let input_ages: Vec<usize> = lines[0]
        .split(",")
        .map(|age| age.parse::<usize>().unwrap())
        .collect();

    let mut ages: [u128; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    for age in input_ages {
        ages[age] += 1;
    }

    for _ in 1..=256 {
        let mut mut_new_ages: [u128; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
        for (age_i, age_count) in ages.iter().enumerate() {
            if age_i == 0 {
                mut_new_ages[8] += age_count;
                mut_new_ages[6] += age_count;
            } else {
                mut_new_ages[age_i - 1] += age_count;
            }
        }

        ages = mut_new_ages;
    }

    let number_of_fish: u128 = ages.iter().map(|age| *age as u128).sum();

    println!("{}", number_of_fish);
}
