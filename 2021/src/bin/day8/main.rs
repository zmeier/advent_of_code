use std::collections::HashMap;
use std::collections::HashSet;

const INPUT_FILE_PATH: &str = "src/bin/day8/input";

/*
 * 0    a   b   c       e   f   g (6)
 * 9    a   b   c   d       f   g (6)
 * 6    a   b       d   e   f   g (6)
 * 3    a       c   d       f   g (5)
 * 2    a       c   d   e       g (5)
 * 5    a   b       d       f   g (5)
 * 1            c           f     (known length 2)
 * 4        b   c   d       f     (known length 4)
 * 7    a       c           f     (known length 3)
 * 8    a   b   c   d   e   f   g (known length 7)
 */

fn main() {
    let lines = lib::lines_from_file(INPUT_FILE_PATH).expect("Could not load input data");
    println!("Part 1");
    part1(&lines);
    println!("Part 2");
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let mut counts: [u32; 10] = [0; 10];

    for line in lines.iter() {
        let pieces: Vec<&str> = line.split(" | ").collect();
        let output: Vec<&str> = pieces[1].split_whitespace().collect();
        for output_val in output.iter() {
            match output_val.len() {
                2 => counts[1] += 1,
                3 => counts[7] += 1,
                4 => counts[4] += 1,
                7 => counts[8] += 1,
                _ => (),
            }
        }
    }

    let sum: u32 = counts.iter().sum();
    println!("{}", sum);
}

fn part2(lines: &Vec<String>) {
    let mut out_sum: u32 = 0;
    for line in lines.iter() {
        let pieces: Vec<&str> = line.split(" | ").collect();
        let input: Vec<&str> = pieces[0].split_whitespace().collect();

        let mut val_to_code: HashMap<usize, HashSet<char>> = HashMap::new();
        let mut code_to_val: HashMap<String, &str> = HashMap::new();

        for input_val in input.iter() {
            let mut input_val_sorted_vec: Vec<char> = input_val.chars().collect();
            input_val_sorted_vec.sort();
            let input_val_sorted: String = input_val_sorted_vec.iter().collect();

            match input_val.len() {
                2 => {
                    val_to_code.insert(1, input_val.chars().collect());
                    code_to_val.insert(input_val_sorted, "1");
                }
                3 => {
                    val_to_code.insert(7, input_val.chars().collect());
                    code_to_val.insert(input_val_sorted, "7");
                }
                4 => {
                    val_to_code.insert(4, input_val.chars().collect());
                    code_to_val.insert(input_val_sorted, "4");
                }
                7 => {
                    val_to_code.insert(8, input_val.chars().collect());
                    code_to_val.insert(input_val_sorted, "8");
                }
                _ => (),
            }
        }

        for input_val in input.iter() {
            let mut input_val_sorted_vec: Vec<char> = input_val.chars().collect();
            input_val_sorted_vec.sort();
            let input_val_sorted: String = input_val_sorted_vec.iter().collect();

            if code_to_val.contains_key(&input_val_sorted) {
                continue;
            }

            let pos_set: HashSet<char> = input_val.chars().collect();
            let str_1_diff: HashSet<&char> =
                val_to_code.get(&1).unwrap().difference(&pos_set).collect();
            let str_4_diff: HashSet<&char> =
                val_to_code.get(&4).unwrap().difference(&pos_set).collect();
            let str_7_diff: HashSet<&char> =
                val_to_code.get(&7).unwrap().difference(&pos_set).collect();

            if input_val.len() == 6 {
                if str_1_diff.len() == 1 {
                    code_to_val.insert(input_val_sorted, "6");
                } else if str_4_diff.len() == 1 {
                    code_to_val.insert(input_val_sorted, "0");
                } else {
                    code_to_val.insert(input_val_sorted, "9");
                }
            } else if input_val.len() == 5 {
                if str_7_diff.len() == 0 {
                    code_to_val.insert(input_val_sorted, "3");
                } else if str_4_diff.len() == 2 {
                    code_to_val.insert(input_val_sorted, "2");
                } else {
                    code_to_val.insert(input_val_sorted, "5");
                }
            }
        }

        let output: Vec<&str> = pieces[1].split_whitespace().collect();
        let mut num_builder: [&str; 4] = [""; 4];
        for (i, code) in output.iter().enumerate() {
            let mut out_sorted_vec: Vec<char> = code.chars().collect();
            out_sorted_vec.sort();
            let out_sorted: String = out_sorted_vec.iter().collect();
            num_builder[i] = *code_to_val.get(&out_sorted).unwrap();
        }

        out_sum += num_builder.join("").parse::<u32>().unwrap();
    }
    println!("{}", out_sum);
}
