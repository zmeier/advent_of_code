const INPUT_FILE_PATH: &str = "src/bin/day3/input";

fn main() {
    let lines = lib::lines_from_file(INPUT_FILE_PATH).expect("Could not load input data");
    println!("Part 1");
    part1(lines.clone());
    println!("Part 2");
    part2(lines.clone());
}

fn part1(lines: Vec<String>) {
    let num_bits = lines[0].len();
    let median: f32 = lines.len() as f32 / 2.0;

    let mut bit_sums: Vec<u32> = vec![0; num_bits];
    for line in lines {
        let line_bytes = line.as_bytes();
        for bit_i in 0..bit_sums.len() {
            bit_sums[bit_i] += line_bytes[bit_i] as u32 - '0' as u32
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    let base: i32 = 2;
    for bit_i in 0..bit_sums.len() {
        let bit_value = base.pow(bit_sums.len() as u32 - bit_i as u32 - 1);
        if bit_sums[bit_i] as f32 >= median {
            gamma += bit_value;
        } else {
            epsilon += bit_value;
        }
    }

    println!("{}", gamma * epsilon);
}

fn part2(lines: Vec<String>) {
    let mut oxygen_vec: Vec<Vec<u32>> = get_data(&lines);
    let num_bits = oxygen_vec[0].len();

    for bit_pos in 0..num_bits {
        let median: f32 = oxygen_vec.len() as f32 / 2.0;
        let mut bit_sum = 0.0;
        for i in 0..oxygen_vec.len() {
            let line = &oxygen_vec[i];
            bit_sum += &(line[bit_pos] as f32);
        }

        let mut new_oxygen_vec: Vec<Vec<u32>> = Vec::new();
        for i in 0..oxygen_vec.len() {
            // Most common is 1
            if bit_sum >= median && oxygen_vec[i][bit_pos] == 1 {
                let new_line_vec = oxygen_vec[i].to_vec();
                new_oxygen_vec.push(new_line_vec);
            }

            // Most common is 0
            if bit_sum < median && oxygen_vec[i][bit_pos] == 0 {
                let new_line_vec = oxygen_vec[i].to_vec();
                new_oxygen_vec.push(new_line_vec);
            }
        }
        oxygen_vec = new_oxygen_vec.to_vec();
        if oxygen_vec.len() == 1 {
            break;
        }
    }

    let mut co2_vec: Vec<Vec<u32>> = get_data(&lines);

    for bit_pos in 0..num_bits {
        let median: f32 = co2_vec.len() as f32 / 2.0;
        let mut bit_sum: f32 = 0.0;
        for i in 0..co2_vec.len() {
            let line = &co2_vec[i];
            bit_sum += &(line[bit_pos] as f32);
        }

        let mut new_co2_vec: Vec<Vec<u32>> = Vec::new();
        for i in 0..co2_vec.len() {
            // Least common is 0
            if bit_sum >= median && co2_vec[i][bit_pos] == 0 {
                let new_line_vec = co2_vec[i].to_vec();
                new_co2_vec.push(new_line_vec);
            }

            // Least common is 1
            if bit_sum < median && co2_vec[i][bit_pos] == 1 {
                let new_line_vec = co2_vec[i].to_vec();
                new_co2_vec.push(new_line_vec);
            }
        }
        co2_vec = new_co2_vec.to_vec();
        if co2_vec.len() == 1 {
            break;
        }
    }

    println!("{:?} {:?}", oxygen_vec, co2_vec);

    let oxy = bit_vec_to_number(oxygen_vec[0].to_vec());
    let co2 = bit_vec_to_number(co2_vec[0].to_vec());

    println!("{}", oxy * co2);
}

fn bit_vec_to_number(bit_vec: Vec<u32>) -> u32 {
    let base: u32 = 2;
    let mut output = 0;
    for bit_i in 0..bit_vec.len() {
        let bit_val = bit_vec[bit_i];
        output += bit_val * base.pow(bit_vec.len() as u32 - bit_i as u32 - 1);
    }
    return output;
}

fn get_data(lines: &Vec<String>) -> Vec<Vec<u32>> {
    let mut vec_of_vec: Vec<Vec<u32>> = Vec::new();
    for line in lines {
        let mut line_vec: Vec<u32> = Vec::new();
        let line_bytes = line.as_bytes();
        for i in 0..line.len() {
            line_vec.push(line_bytes[i] as u32 - '0' as u32);
        }
        vec_of_vec.push(line_vec);
    }
    return vec_of_vec;
}
