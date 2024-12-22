use std::collections::{VecDeque, HashMap};

fn mix_secret(next: i64, secret: i64) -> i64 {
    next ^ secret
}

fn prune_secret(number: i64) -> i64 {
    number % 16777216
}

fn next_secret(secret: i64) -> i64 {
    let mut step1 = secret << 6;
    step1 = mix_secret(step1, secret);
    step1 = prune_secret(step1);

    let mut step2 = step1 >> 5;
    step2 = mix_secret(step2, step1);
    step2 = prune_secret(step2);

    let mut step3 = step2 << 11;
    step3 = mix_secret(step3, step2);
    step3 = prune_secret(step3);

    step3
}

const GENERATIONS: i64 = 2000;

#[allow(dead_code)]
pub fn part1() -> String {
    let mut secrets = aoc::to_lines("input/day22.txt")
        .into_iter()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    for _ in 0..GENERATIONS {
        for secret in secrets.iter_mut() {
            *secret = next_secret(*secret);
        }
    }

    secrets.iter().sum::<i64>().to_string()
}

fn get_prices_and_changes(mut secret: i64) -> (Vec<i64>, Vec<i64>) {
    let mut prices : Vec<i64> = Vec::new();
    prices.push(secret);
    let mut changes : Vec<i64> = Vec::new();

    for _ in 0..GENERATIONS {
        let price = secret % 10;
        secret = next_secret(secret);
        let next_price = secret % 10;
        let change = next_price - price;

        prices.push(next_price);
        changes.push(change);
    }

    (prices, changes)
}

#[allow(dead_code)]
pub fn part2() -> String {
    let mut secrets = aoc::to_lines("input/day22.txt")
        .into_iter()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    // not 2103, not 2105, not 2124
    /*let mut best_seq_values: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();
    for secret in &mut secrets {
        let (prices, changes) = get_prices_and_changes(*secret);
        let mut seq_values: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();

        for c in 0..changes.len() - 3 {
            let price = prices[c + 4];
            let seq = (changes[c], changes[c + 1], changes[c + 2], changes[c + 3]);

            if let Some(prev_price) = seq_values.get_mut(&seq) {
                if *prev_price < price {
                    *prev_price = price;
                }
            }
            else {
                seq_values.insert(seq, price);
            }
        }

        for (seq, price) in seq_values {
            if let Some(best_price) = best_seq_values.get_mut(&seq) {
                *best_price += price;
            }
            else {
                best_seq_values.insert(seq, price);
            }
        }
    }

    let mut most_bananas = 0;
    for (_, price) in best_seq_values {
        if price > most_bananas {
            most_bananas = price;
        }
    }

    most_bananas.to_string()*/

    // 2140
    let mut best_seq_values: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();
    for secret in &mut secrets {
        let mut seq_values: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();
        let mut changes: VecDeque<i64> = VecDeque::new();

        for _ in 0..GENERATIONS {
            let price = *secret % 10;
            *secret = next_secret(*secret);
            let next_price = *secret % 10;

            let change = next_price - price;

            changes.push_back(change);
            if changes.len() < 4 {
                continue;
            }

            let seq = (changes[0], changes[1], changes[2], changes[3]);
            if let Some(seq_price) = seq_values.get_mut(&seq) {
                // Ignore
            }
            else {
                seq_values.insert(seq, next_price);
            }

            changes.pop_front();
        }

        for (seq, secret_price) in &seq_values {
            if let Some(price) = best_seq_values.get_mut(seq) {
                *price += secret_price;
            }
            else {
                best_seq_values.insert(*seq, *secret_price);
            }
        }
    }

    let mut max_bananas = 0;
    for (_, bananas) in &best_seq_values {
        if *bananas > max_bananas {
            max_bananas = *bananas;
        }
    }

    max_bananas.to_string()
}