use super::utils;

use std::collections::HashSet;

fn parse_text(contents: &str) -> Vec<i32> {
    contents.split("\n").map(|e| e.parse().unwrap()).collect()
}

pub fn run() {
    let filename = "inputs/01/input.txt";
    let contents: String = utils::read_input(&filename);
    let entries = parse_text(&contents);

    let mut cumulative: Vec<i32> = Vec::new();
    let mut acc: i32 = 0;
    let mut seen: HashSet<i32> = HashSet::new();
    let mut seen_twice: Option<i32> = None;

    for x in entries.iter().cycle() {
        acc = acc + x;
        cumulative.push(acc);
        if let None = seen_twice {
            if seen.contains(&acc) {
                println!("First seen twice: {:?}", acc);
                seen_twice = Some(acc);
            };
        } else if cumulative.len() >= entries.len() {
            break;
        };
        seen.insert(acc);
    }

    println!("End value: {:?}", cumulative[entries.len() - 1]);
}
