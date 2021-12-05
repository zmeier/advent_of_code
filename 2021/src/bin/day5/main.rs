use std::collections::HashMap;

const INPUT_FILE_PATH: &str = "src/bin/day5/input";

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Coordinate {
    x: u32,
    y: u32,
}

fn main() {
    let lines = lib::lines_from_file(INPUT_FILE_PATH).expect("Could not load input data");
    println!("Part 1");
    part1(&lines);
    println!("Part 2");
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let mut line_coords: HashMap<Coordinate, u32> = HashMap::new();

    for line in lines.iter() {
        let (start_coord, end_coord) = parse_line_into_coordinates(line);

        if start_coord.x == end_coord.x {
            let start_y = std::cmp::min(start_coord.y, end_coord.y);
            let end_y = std::cmp::max(start_coord.y, end_coord.y);

            for y in start_y..=end_y {
                add_new_coordinate(start_coord.x, y, &mut line_coords);
            }
        } else if start_coord.y == end_coord.y {
            let start_x = std::cmp::min(start_coord.x, end_coord.x);
            let end_x = std::cmp::max(start_coord.x, end_coord.x);

            for x in start_x..=end_x {
                add_new_coordinate(x, start_coord.y, &mut line_coords);
            }
        }
    }
    let total_overlapping_points: u32 = count_number_overlapping_points(line_coords);

    println!("{}", total_overlapping_points);
}

fn part2(lines: &Vec<String>) {
    let mut line_coords: HashMap<Coordinate, u32> = HashMap::new();

    for line in lines.iter() {
        let (start_coord, end_coord) = parse_line_into_coordinates(line);

        if start_coord.x == end_coord.x {
            let start_y = std::cmp::min(start_coord.y, end_coord.y);
            let end_y = std::cmp::max(start_coord.y, end_coord.y);

            for y in start_y..=end_y {
                add_new_coordinate(start_coord.x, y, &mut line_coords);
            }
        } else if start_coord.y == end_coord.y {
            let start_x = std::cmp::min(start_coord.x, end_coord.x);
            let end_x = std::cmp::max(start_coord.x, end_coord.x);

            for x in start_x..=end_x {
                add_new_coordinate(x, start_coord.y, &mut line_coords);
            }
        } else {
            let slope = (end_coord.y as f32 - start_coord.y as f32)
                / (end_coord.x as f32 - start_coord.x as f32);
            if slope == 1.0 {
                for i in 0..=(end_coord.x - start_coord.x) {
                    add_new_coordinate(start_coord.x + i, start_coord.y + i, &mut line_coords);
                }
            } else if slope == -1.0 {
                for i in 0..=(end_coord.x - start_coord.x) {
                    add_new_coordinate(start_coord.x + i, start_coord.y - i, &mut line_coords);
                }
            }
        }
    }
    let total_overlapping_points: u32 = count_number_overlapping_points(line_coords);

    println!("{}", total_overlapping_points);
}

fn parse_line_into_coordinates(line: &str) -> (Coordinate, Coordinate) {
    let coordinate_pieces: Vec<Coordinate> = line
        .split(" -> ")
        .map(|coord| coord.split(",").collect())
        .map(|coord: Vec<&str>| Coordinate {
            x: coord[0].parse::<u32>().unwrap(),
            y: coord[1].parse::<u32>().unwrap(),
        })
        .collect();
    if coordinate_pieces[0].x <= coordinate_pieces[1].x {
        return (coordinate_pieces[0].clone(), coordinate_pieces[1].clone());
    }
    (coordinate_pieces[1].clone(), coordinate_pieces[0].clone())
}

fn add_new_coordinate(x: u32, y: u32, line_coords: &mut HashMap<Coordinate, u32>) {
    let coord = Coordinate { x: x, y: y };
    if line_coords.contains_key(&coord) {
        let updated_val = line_coords.get(&coord).unwrap() + 1;
        line_coords.insert(coord, updated_val);
    } else {
        line_coords.insert(coord, 1);
    }
}

fn count_number_overlapping_points(line_coords: HashMap<Coordinate, u32>) -> u32 {
    let mut total_overlapping_points: u32 = 0;
    for (_, points) in line_coords {
        if points > 1 {
            total_overlapping_points += 1;
        }
    }
    total_overlapping_points
}
