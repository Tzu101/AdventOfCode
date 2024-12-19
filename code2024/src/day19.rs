use std::collections::{HashMap, HashSet};
use regex::Regex;

#[allow(dead_code)]
pub fn part1() -> String {
    let mut towel_data = aoc::to_lines("input/day19.txt");

    let available_towels = towel_data[0].clone();
    let available_towels = available_towels.split(", ").collect::<Vec<&str>>();
    towel_data.drain(0..2);
    let requested_towels = towel_data;

    let available_towels = available_towels.iter().map(|towel| format!("({towel})*")).collect::<Vec<String>>();

    let mut possible_towels = 0;
    for requested_towel in requested_towels {
        let towel_pattern = format!(r"^({}){{{}}}$", available_towels.join("|"), requested_towel.len());
        let towel_regex = Regex::new(&towel_pattern).unwrap();

        if let Some(matched) = towel_regex.find(&requested_towel) {
            let match_length = matched.end() - matched.start(); // Calculate length
            println!("Matched text: '{}', Length: {}", matched.as_str(), match_length);
            possible_towels += 1;
        } else {
            println!("No match found.");
        }
    }

    possible_towels.to_string()
}

fn num_of_patterns(towel: &str, parts: &[&str], towel_map: &mut HashMap<String, u64>) -> u64 {
    if towel_map.contains_key(towel) {
        return towel_map[towel];
    }

    if towel.len() == 0 {
        return 1;
    }

    let mut total_patterns = 0;
    for part in parts {
        if towel.len() < part.len() {
            continue;
        }

        if &towel[..part.len()] != *part {
            continue;
        }

        let sub_towel = &towel[part.len()..];
        let towel_matches = num_of_patterns(sub_towel, &parts, towel_map);
        towel_map.insert(sub_towel.to_string(), towel_matches);
        total_patterns += towel_matches;
    }
    total_patterns
}

#[allow(dead_code)]
pub fn part2() -> String {
    let mut towel_data = aoc::to_lines("input/day19.txt");

    let available_towels = towel_data[0].clone();
    let available_towels = available_towels.split(", ").collect::<Vec<&str>>();
    towel_data.drain(0..2);
    let requested_towels = towel_data;

    let mut towel_map = HashMap::new();
    let mut possible_towels = 0u64;
    let mut unique_towels = 0;
    for requested_towel in requested_towels {
        let num = num_of_patterns(&requested_towel, &available_towels, &mut towel_map);
        possible_towels += num;
        if num != 0 {
            unique_towels += 1;
        }
        println!("'{}' matched: {}", requested_towel, num);
    }
    println!("Unique: {unique_towels}");

    possible_towels.to_string()
}