use std::collections::HashMap;

#[allow(dead_code)]
pub fn part1() -> String {
    let input = &aoc::to_lines("input/day5.txt");
    let input_split = input.iter().position(|r| r == "").unwrap();

    let rules = &input[0..input_split];
    let mut rule_map: HashMap<u32, Vec<u32>> = HashMap::new();
    for rule in rules {
        let nums = rule.split("|").collect::<Vec<&str>>();
        let first = nums[0].parse::<u32>().unwrap();
        let second = nums[1].parse::<u32>().unwrap();
        if let Some(rule) = rule_map.get_mut(&first) {
            rule.push(second);
        }
        else {
            rule_map.insert(first, Vec::new());
            rule_map.get_mut(&first).unwrap().push(second);
        }
    }

    let examples = (&input[(input_split+1)..input.len()]).iter()
        .map(|s| {
            s.split(',')
                .filter_map(|num| num.parse::<u32>().ok()) // Parse to u32, ignore errors
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut middle_sum = 0;
    for example in examples {
        let mut valid = true;
        'nested_for: for f in 1..example.len() {
            let first = example[f];
            if let Some(first_rules) = rule_map.get(&first) {
                for s in 0..f {
                    let second = example[s];
                    if first_rules.contains(&second) {
                        valid = false;
                        break 'nested_for;
                    }
                }
            }
        }

        if valid {
            middle_sum += example[example.len() / 2];
        }
    }
    middle_sum.to_string()
}

#[allow(dead_code)]
pub fn part2() -> String {
    let mut result = 0;
    result.to_string()
}