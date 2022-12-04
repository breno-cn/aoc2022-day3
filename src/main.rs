use std::collections::HashSet;

fn parse_input(filepath: &str) -> Vec<String> {
    std::fs::read_to_string(filepath)
        .unwrap()
        .split("\n")
        .map(String::from)
        .collect::<Vec<String>>()
}

fn repeats(first: &String, second: &String) -> Option<char> {
    let mut chars = HashSet::new();

    for c in first.chars() {
        chars.insert(c.clone());
    }

    for c in second.chars() {
        if chars.contains(&c) {
            return Some(c);
        }
    }

    None
}

fn char_to_value(c: &char) -> i32 {
    if c.is_lowercase() {
        (*c as i32) - ('a' as i32) + 1
    } else {
        (*c as i32) - ('A' as i32) + 26 + 1
    }
}

fn solve(rucksacks: Vec<String>) -> i32 {
    rucksacks.iter()
        .map(|rucksack| {
            let half = rucksack.len() / 2;
            let (first, second) = rucksack.split_at(half);

            let (first, second) = (first.to_string(), second.to_string());
            char_to_value(&repeats(&first, &second).unwrap())
        })
        .sum()
}

fn main() {
    let input = parse_input("input.txt");

    let result = solve(input);
    println!("{:}", result);
}
