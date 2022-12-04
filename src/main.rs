use std::collections::{HashSet, HashMap};

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

fn solve(rucksacks: &Vec<String>) -> i32 {
    rucksacks.iter()
        .map(|rucksack| {
            let half = rucksack.len() / 2;
            let (first, second) = rucksack.split_at(half);

            let (first, second) = (first.to_string(), second.to_string());
            char_to_value(&repeats(&first, &second).unwrap())
        })
        .sum()
}

fn group_by_three(rucksacks: &Vec<String>) -> Vec<Vec<&String>> {
    let mut result = vec![];
    let mut i = 0;

    let mut curr = vec![];
    for rucksack in rucksacks {
        curr.push(rucksack);

        i += 1;
        if i % 3 == 0 {
            result.push(curr.clone());
            curr.clear();
        }
    }

    result
}

fn find_badge(rucksacks: &Vec<&String>) -> char {
    let unique_chars = rucksacks.iter()
        .map(|rucksack| {
            let mut set = HashSet::new();

            rucksack.chars().for_each(|c| { set.insert(c); });

            set
        })
        .collect::<Vec<HashSet<char>>>();

    let mut char_count = HashMap::new();
    for set in unique_chars {
        for c in set {
            if !char_count.contains_key(&c) {
                char_count.insert(c, 1);
            } else {
                let previous = char_count.get(&c).unwrap();

                char_count.insert(c, previous + 1);
            }
        }
    }

    let res = char_count.iter()
        .find(|(_k, v)| **v == 3)
        .unwrap().0;

    *res
}

fn solve_part2(rucksacks: &Vec<String>) -> i32 {
    let groups = group_by_three(rucksacks);

    groups.iter()
        .map(find_badge)
        .map(|badge| char_to_value(&badge))
        .sum()
}

fn main() {
    let input = parse_input("input.txt");

    let result = solve(&input);
    println!("{:}", result);

    let result = solve_part2(&input);
    println!("{:}", result);
}
