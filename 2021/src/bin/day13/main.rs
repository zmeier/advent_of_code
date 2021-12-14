use std::collections::HashSet;

const INPUT_FILE_PATH: &str = "src/bin/day13/input";

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Point {
    x: usize,
    y: usize,
}

fn main() {
    let lines = lib::lines_from_file(INPUT_FILE_PATH).expect("Could not load input data");
    println!("Part 1");
    part1(&lines);
    println!("Part 2");
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let (points, folds) = parse_input(lines);

    let first_fold: &Point = &folds[0];
    let folded_points = fold(&points, first_fold);

    println!("{}", folded_points.len());
}

fn part2(lines: &Vec<String>) {
    let (mut points, folds) = parse_input(lines);

    for fold_point in folds.iter() {
        points = fold(&points, fold_point);
    }

    print_points(&points);
}

fn parse_input(lines: &Vec<String>) -> (HashSet<Point>, Vec<Point>) {
    let mut points: HashSet<Point> = HashSet::new();
    let mut folds: Vec<Point> = Vec::new();

    for line in lines.iter() {
        if line == "" {
            continue;
        } else if line.starts_with("fold along x=") {
            folds.push(Point {
                x: line
                    .strip_prefix("fold along x=")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
                y: 0,
            });
        } else if line.starts_with("fold along y=") {
            folds.push(Point {
                x: 0,
                y: line
                    .strip_prefix("fold along y=")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
            });
        } else {
            let coords: Vec<usize> = line
                .split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            points.insert(Point {
                x: coords[0],
                y: coords[1],
            });
        }
    }

    (points, folds)
}

fn fold(points: &HashSet<Point>, fold: &Point) -> HashSet<Point> {
    let mut folded_points: HashSet<Point> = HashSet::new();
    for point in points.iter() {
        if fold.x == 0 {
            if point.y == fold.y {
                // at fold so remove
                continue;
            } else if point.y > fold.y {
                folded_points.insert(Point {
                    x: point.x,
                    y: 2 * fold.y - point.y,
                });
            } else {
                folded_points.insert(point.clone());
            }
        } else if fold.y == 0 {
            if point.x == fold.x {
                // at fold so remove
                continue;
            } else if point.x > fold.x {
                folded_points.insert(Point {
                    x: 2 * fold.x - point.x,
                    y: point.y,
                });
            } else {
                folded_points.insert(point.clone());
            }
        }
    }

    folded_points
}

fn print_points(points: &HashSet<Point>) {
    let mut max_x = 0;
    let mut max_y = 0;

    for point in points.iter() {
        if point.x > max_x {
            max_x = point.x;
        } else if point.y > max_y {
            max_y = point.y;
        }
    }

    let mut grid: Vec<Vec<char>> = vec![vec!['.'; max_x + 1]; max_y + 1];
    for point in points.iter() {
        grid[point.y][point.x] = '#';
    }

    for row in grid {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}
