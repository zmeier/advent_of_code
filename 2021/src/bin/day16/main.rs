const INPUT_FILE_PATH: &str = "src/bin/day16/input";

fn main() {
    let lines = lib::lines_from_file(INPUT_FILE_PATH).expect("Could not load input data");
    println!("Part 1");
    part1(&lines);
    println!("Part 2");
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let data_as_bit_arrays: Vec<[u128; 4]> = lines
        .iter()
        .next()
        .unwrap()
        .chars()
        .map(|c| to_binary(c))
        .collect();
    let data: Vec<u128> = data_as_bit_arrays.concat();

    let (_, version_sum, _) = read_packet(&data, 0);

    println!("{:?}", version_sum);
}

fn part2(lines: &Vec<String>) {
    let data_as_bit_arrays: Vec<[u128; 4]> = lines
        .iter()
        .next()
        .unwrap()
        .chars()
        .map(|c| to_binary(c))
        .collect();
    let data: Vec<u128> = data_as_bit_arrays.concat();

    let (_, _, value) = read_packet(&data, 0);

    println!("{:?}", value);
}

fn read_packet(data: &Vec<u128>, initial_pos: usize) -> (usize, u128, u128) {
    let mut version_sum = 0;
    let mut pos = initial_pos;
    let value;
    // get id
    let version = bit_vec_to_number(&data[pos..pos + 3]);
    pos += 3;
    let type_id = bit_vec_to_number(&data[pos..pos + 3]);
    pos += 3;

    if type_id == 4 {
        let mut read_everything = false;
        let mut literal_bits: Vec<u128> = Vec::new();
        while !read_everything {
            // literal value
            if data[pos] == 0 {
                // last group
                pos += 5;
                literal_bits.append(&mut data[pos - 4..pos].to_vec());
                read_everything = true;
            } else {
                pos += 5;
                literal_bits.append(&mut data[pos - 4..pos].to_vec());
            }
        }
        value = bit_vec_to_number(&literal_bits);
    } else {
        let length_type_id = data[pos];
        pos += 1;
        let mut sub_values: Vec<u128> = Vec::new();
        // bit of 0 means 15 for length, else 11
        if length_type_id == 0 {
            // dictates length of subpackets
            let length_of_sub_packets = bit_vec_to_number(&data[pos..pos + 15]) as usize;
            pos += 15;
            let mut consumed_length: usize = 0;
            while consumed_length < length_of_sub_packets {
                let (new_pos, sub_packet_version_sum, sub_value) = read_packet(data, pos);
                consumed_length += new_pos - pos;
                pos = new_pos;
                version_sum += sub_packet_version_sum;
                sub_values.push(sub_value);
            }
        } else {
            // dictates number of subpackets
            let num_sub_packets = bit_vec_to_number(&data[pos..pos + 11]);
            pos += 11;
            for _ in 0..num_sub_packets {
                let (new_pos, sub_packet_version_sum, sub_value) = read_packet(data, pos);
                pos = new_pos;
                version_sum += sub_packet_version_sum;
                sub_values.push(sub_value);
            }
        }
        value = operate(&sub_values, type_id);
    }

    version_sum += version;

    (pos, version_sum, value)
}

fn to_binary(c: char) -> [u128; 4] {
    match c {
        '0' => [0, 0, 0, 0],
        '1' => [0, 0, 0, 1],
        '2' => [0, 0, 1, 0],
        '3' => [0, 0, 1, 1],
        '4' => [0, 1, 0, 0],
        '5' => [0, 1, 0, 1],
        '6' => [0, 1, 1, 0],
        '7' => [0, 1, 1, 1],
        '8' => [1, 0, 0, 0],
        '9' => [1, 0, 0, 1],
        'A' => [1, 0, 1, 0],
        'B' => [1, 0, 1, 1],
        'C' => [1, 1, 0, 0],
        'D' => [1, 1, 0, 1],
        'E' => [1, 1, 1, 0],
        'F' => [1, 1, 1, 1],
        _ => [1, 1, 1, 1], // should never happen
    }
}

fn operate(vals: &Vec<u128>, operation: u128) -> u128 {
    match operation {
        0 => vals.iter().sum(),
        1 => {
            let mut product = 1;
            for val in vals.iter() {
                product *= val;
            }
            product
        }
        2 => *vals.iter().min().unwrap(),
        3 => *vals.iter().max().unwrap(),
        5 => {
            if vals[0] > vals[1] {
                return 1;
            }
            0
        }
        6 => {
            if vals[0] < vals[1] {
                return 1;
            }
            0
        }
        7 => {
            if vals[0] == vals[1] {
                return 1;
            }
            0
        }
        _ => 0,
    }
}

fn bit_vec_to_number(bit_vec: &[u128]) -> u128 {
    let mut output = 0;
    for bit_i in 0..bit_vec.len() {
        let bit_val = bit_vec[bit_i];
        output += bit_val * bit_pow(bit_vec.len() - bit_i - 1);
    }
    output
}

fn bit_pow(bit_pos: usize) -> u128 {
    let base: u128 = 2;
    base.pow(bit_pos as u32)
}
