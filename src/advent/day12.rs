use std::collections::{HashMap, HashSet};

use anyhow::Result;

use super::input;

pub fn print_result(input: impl std::io::Read) -> Result<()> {
    let connections = parse_input(input)?;

    let first_cave = "start".to_owned();
    let routes = find_routes(&connections, vec![], &first_cave, false);

    println!("Part 1: Number of paths through caves = {}", routes.len());

    let routes = find_routes(&connections, vec![], &first_cave, true);

    println!("Part 2: Number of paths with revist = {}", routes.len());

    Ok(())
}

type Cave = String;
type Route = Vec<Cave>;

fn parse_input(input: impl std::io::Read) -> Result<HashMap<Cave, Vec<Cave>>> {
    let lines = input::get_input_lines(input)?;
    let mut connections: HashMap<Cave, Vec<Cave>> = HashMap::new();
    for line in lines {
        let (a, b) = line
            .split_once('-')
            .map(|(a, b)| (a.to_owned(), b.to_owned()))
            .ok_or(anyhow::Error::msg("malformed input"))?;

        connections
            .entry(a.clone())
            .or_insert(Vec::new())
            .push(b.clone());
        connections.entry(b).or_insert(Vec::new()).push(a);
    }

    Ok(connections)
}

fn find_routes(
    conns: &HashMap<Cave, Vec<Cave>>,
    mut visited: Route,
    next_cave: &Cave,
    allow_revisit: bool,
) -> Vec<Route> {
    visited.push(next_cave.clone());
    match visited.last().unwrap().as_str() {
        "end" => vec![visited],
        current_cave => {
            let mut routes: Vec<Route> = vec![];
            for next_cave in conns.get(current_cave).unwrap() {
                let new_cave = !visited.contains(next_cave);
                let big_cave = next_cave.chars().all(char::is_uppercase);
                let can_revisit =
                    allow_revisit && next_cave != "start" && !have_revisited(&visited);

                if (new_cave || big_cave) || can_revisit {
                    // clone visited to ensure every potential path can visit every cave
                    let visited = visited.clone();
                    routes.append(&mut find_routes(conns, visited, next_cave, allow_revisit))
                }
            }
            routes
        }
    }
}

fn have_revisited(route_taken: &[Cave]) -> bool {
    let mut visited: HashSet<&Cave> = HashSet::new();

    for cave in route_taken {
        if !cave.chars().all(char::is_uppercase) {
            if visited.contains(cave) {
                return true;
            }
            visited.insert(cave);
        }
    }
    false
}
