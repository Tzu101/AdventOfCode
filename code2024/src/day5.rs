use std::collections::HashMap;

fn get_rules_and_examples() -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
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

    (rule_map, examples)
}

enum Rules {
    Follows,
    Breaks(usize, usize)
}
fn example_follows_rules(example: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> Rules {
    for f in 1..example.len() {
        let first = example[f];
        if let Some(first_rules) = rules.get(&first) {
            for s in 0..f {
                let second = example[s];
                if first_rules.contains(&second) {
                    return Rules::Breaks(f, s);
                }
            }
        }
    }
    Rules::Follows
}

#[allow(dead_code)]
pub fn part1() -> String {
    let (rules, examples) = get_rules_and_examples();

    let mut middle_sum = 0;
    for example in examples {
        match example_follows_rules(&example, &rules) {
            Rules::Follows => {
                middle_sum += example[example.len() / 2];
            }
            _ => {}
        }
    }
    middle_sum.to_string()
}

#[allow(dead_code)]
pub fn part2() -> String {
    let (rules, mut examples) = get_rules_and_examples();

    let mut middle_sum = 0;
    'fix_example: for example in &mut examples {
        let mut already_follows = true;
        loop {
            match example_follows_rules(&example, &rules) {
                Rules::Follows => {
                    if !already_follows {
                        middle_sum += example[example.len() / 2];
                    }
                    continue 'fix_example;
                }
                Rules::Breaks(first, second) => {
                    already_follows = false;
                    let breaks_rules = example.remove(second);
                    example.insert(first, breaks_rules);
                }
            }
        }
    }
    middle_sum.to_string()
}