const INPUT_FILE_PATH: &str = "src/bin/day3/input";

fn main() {
    let lines = lib::lines_from_file(INPUT_FILE_PATH).expect("Could not load input data");
    println!("Part 1");
    part1(&lines);
    println!("Part 2");
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let bit_vec = get_bit_vector_from_lines(&lines);
    let median: f32 = bit_vec.len() as f32 / 2.0;
    let num_bits = bit_vec[0].len();

    let mut bit_sums: Vec<u32> = vec![0; num_bits];
    for bits in bit_vec {
        for (bit_i, bit) in bits.iter().enumerate() {
            bit_sums[bit_i] += bit;
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    for (bit_i, bit_sum) in bit_sums.iter().enumerate() {
        let bit_value = bit_pow(bit_sums.len() - bit_i - 1);
        if *bit_sum as f32 >= median {
            gamma += bit_value;
        } else {
            epsilon += bit_value;
        }
    }

    println!("{}*{}={}", gamma, epsilon, gamma * epsilon);
}

fn part2(lines: &Vec<String>) {
    let o2_rating = get_rating(&lines, 1, 0);
    let co2_rating = get_rating(&lines, 0, 1);

    println!("{}*{}={}", o2_rating, co2_rating, o2_rating * co2_rating);
}

fn get_rating(lines: &Vec<String>, bit_val_for_1_max: u32, bit_val_for_0_max: u32) -> u32 {
    let mut bit_vec: Vec<Vec<u32>> = get_bit_vector_from_lines(&lines);
    let num_bits = bit_vec[0].len();

    for bit_pos in 0..num_bits {
        let median: f32 = bit_vec.len() as f32 / 2.0;
        let mut bit_sum: f32 = 0.0;
        for bits in bit_vec.iter() {
            bit_sum += bits[bit_pos] as f32;
        }

        let mut new_bit_vec: Vec<Vec<u32>> = Vec::new();
        for bits in bit_vec.iter() {
            // Most common is 1
            if bit_sum >= median {
                if bits[bit_pos] == bit_val_for_1_max {
                    new_bit_vec.push(bits.clone());
                }
            } else {
                if bits[bit_pos] == bit_val_for_0_max {
                    new_bit_vec.push(bits.clone());
                }
            }
        }
        bit_vec = new_bit_vec;
        if bit_vec.len() == 1 {
            break;
        }
    }

    bit_vec_to_number(&bit_vec[0])
}

fn get_bit_vector_from_lines(lines: &Vec<String>) -> Vec<Vec<u32>> {
    let mut vec_of_vec: Vec<Vec<u32>> = Vec::new();
    for line in lines {
        let mut line_vec: Vec<u32> = Vec::new();
        let line_bytes = line.as_bytes();
        for i in 0..line.len() {
            line_vec.push(line_bytes[i] as u32 - '0' as u32);
        }
        vec_of_vec.push(line_vec);
    }
    vec_of_vec
}

fn bit_pow(bit_pos: usize) -> u32 {
    let base: u32 = 2;
    base.pow(bit_pos as u32)
}

fn bit_vec_to_number(bit_vec: &Vec<u32>) -> u32 {
    let mut output = 0;
    for bit_i in 0..bit_vec.len() {
        let bit_val = bit_vec[bit_i];
        output += bit_val * bit_pow(bit_vec.len() - bit_i - 1);
    }
    output
}