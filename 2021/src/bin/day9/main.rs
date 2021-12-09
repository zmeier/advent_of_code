const INPUT_FILE_PATH: &str = "src/bin/day9/input";
const MOVE_MASK: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct MapPoint {
    val: u32,
    in_basin: bool,
}

fn main() {
    let lines = lib::lines_from_file(INPUT_FILE_PATH).expect("Could not load input data");
    println!("Part 1");
    part1(&lines);
    println!("Part 2");
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let map = lines_to_map(&lines);
    let low_points = get_low_points(&map);

    let mut risk_level_sum: u32 = 0;
    for point in low_points.iter() {
        risk_level_sum += map[point.x as usize][point.y as usize].val + 1;
    }

    println!("{}", risk_level_sum);
}

fn part2(lines: &Vec<String>) {
    let mut map = lines_to_map(&lines);
    let low_points = get_low_points(&map);

    let mut basin_sizes: Vec<u32> = Vec::new();
    for low_point in low_points.iter() {
        let basin_size = find_basin(low_point, &mut map);
        basin_sizes.push(basin_size);
    }

    basin_sizes.sort_by(|a, b| b.cmp(a));
    let mut output: u32 = 1;
    for i in 0..3 {
        output *= basin_sizes[i];
    }

    println!("{}", output);
}

fn lines_to_map(lines: &Vec<String>) -> Vec<Vec<MapPoint>> {
    let mut map: Vec<Vec<MapPoint>> = Vec::new();

    for line in lines.iter() {
        let map_line: Vec<MapPoint> = line
            .chars()
            .map(|c| MapPoint {
                val: c.to_digit(10).unwrap(),
                in_basin: false,
            })
            .collect();
        map.push(map_line);
    }

    map
}

fn get_low_points(map: &Vec<Vec<MapPoint>>) -> Vec<Point> {
    let height = map.len();
    let width = map[0].len();
    let mut points: Vec<Point> = Vec::new();
    for i in 0..height {
        for j in 0..width {
            let cell_val: u32 = map[i][j].val;
            if i > 0 && cell_val >= map[i - 1][j].val {
                continue;
            }
            if i < height - 1 && cell_val >= map[i + 1][j].val {
                continue;
            }
            if j > 0 && cell_val >= map[i][j - 1].val {
                continue;
            }
            if j < width - 1 && cell_val >= map[i][j + 1].val {
                continue;
            }
            points.push(Point { x: i, y: j });
        }
    }

    points
}

fn find_basin(low_point: &Point, map: &mut Vec<Vec<MapPoint>>) -> u32 {
    // Make sure we are in bounds of the map
    if low_point.x >= map.len() || low_point.y >= map[0].len() {
        return 0;
    }
    // Check if already found point or is a boundary
    if map[low_point.x][low_point.y].in_basin || map[low_point.x][low_point.y].val == 9 {
        return 0;
    }

    map[low_point.x][low_point.y].in_basin = true;
    let mut size: u32 = 1;
    for mask in MOVE_MASK.iter() {
        let (move_x, move_y) = mask;
        let new_x: isize = low_point.x as isize + move_x;
        let new_y: isize = low_point.y as isize + move_y;
        // Negative numbers cannot be created because it will go outside the bounds of usize data type
        if new_x >= 0 && new_y >= 0 {
            size += find_basin(
                &Point {
                    x: new_x as usize,
                    y: new_y as usize,
                },
                map,
            );
        }
    }

    size
}
