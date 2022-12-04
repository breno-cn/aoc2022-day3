use std::collections::HashMap;

fn parse_input(filepath: &str) -> Vec<(String, String)> {
    std::fs::read_to_string(filepath)
        .unwrap()
        .split("\n")
        .map(|line| {
            let half = line.len() / 2;
            let (first, second) = line.split_at(half);

            (first.to_string(), second.to_string())
        })
        .collect::<Vec<(String, String)>>()
}

fn repeats(first: &String, second: &String) -> Option<char> {
    let mut char_count: HashMap<char, bool> = HashMap::new();

    for c in first.chars() {
        char_count.insert(c.clone(), true);
    }

    for c in second.chars() {
        if char_count.contains_key(&c) {
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

fn solve(rucksacks: Vec<(String, String)>) -> i32 {
    rucksacks.iter()
        .map(|(first, second)| {
            char_to_value(&repeats(first, second).unwrap())
        })
        .sum()
}

fn main() {
    let input = parse_input("input.txt");

    let result = solve(input);
    println!("{:}", result);
}
