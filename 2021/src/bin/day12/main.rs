use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

const INPUT_FILE_PATH: &str = "src/bin/day12/input";

fn main() {
    let lines = lib::lines_from_file(INPUT_FILE_PATH).expect("Could not load input data");
    println!("Part 1");
    part1(&lines);
    println!("Part 2");
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let mut graph: HashMap<&str, Vec<&str>> = build_graph(lines);

    let mut visited: HashSet<String> = HashSet::new();
    let num_paths = find_paths(&mut graph, "start", &mut visited, false);

    println!("{}", num_paths);
}

fn part2(lines: &Vec<String>) {
    let mut graph: HashMap<&str, Vec<&str>> = build_graph(lines);

    let mut visited: HashSet<String> = HashSet::new();
    let num_paths = find_paths(&mut graph, "start", &mut visited, true);

    println!("{}", num_paths);
}

fn build_graph(lines: &Vec<String>) -> HashMap<&str, Vec<&str>> {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in lines.iter() {
        let pieces: Vec<&str> = line.split("-").collect();
        let start = pieces[0];
        let end = pieces[1];

        if start != "end" && end != "start" {
            if graph.contains_key(start) {
                graph.get_mut(start).unwrap().push(end);
            } else {
                graph.insert(start, vec![end]);
            }
        }

        if start != "start" && end != "end" {
            if graph.contains_key(end) {
                graph.get_mut(end).unwrap().push(start);
            } else {
                graph.insert(end, vec![start]);
            }
        }
    }

    graph
}

fn find_paths(
    graph: &mut HashMap<&str, Vec<&str>>,
    edge: &str,
    visited: &mut HashSet<String>,
    allow_double_visit: bool,
) -> u32 {
    if edge == "end" {
        return 1;
    }

    let mut new_allow_double_visit = allow_double_visit;
    if edge.chars().nth(0).unwrap().is_lowercase() && visited.contains(edge) {
        if new_allow_double_visit {
            new_allow_double_visit = false;
        } else {
            return 0;
        }
    }

    visited.insert(String::from(edge));

    let mut edges: VecDeque<&str> = VecDeque::from_iter(graph.get_mut(edge).unwrap().clone());
    let mut num_paths: u32 = 0;

    while let Some(edge) = edges.pop_front() {
        num_paths += find_paths(graph, edge, &mut visited.clone(), new_allow_double_visit);
    }

    num_paths
}
