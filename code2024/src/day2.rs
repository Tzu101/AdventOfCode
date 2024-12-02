const MIN_ORDER_DIFF: i32 = 1;
const MAX_ORDER_DIFF: i32 = 3;
fn is_ordered(prev: i32, next: i32) -> bool {
    let diff: i32 = (next - prev) as i32;
    MIN_ORDER_DIFF <= diff && diff <= MAX_ORDER_DIFF
}

#[allow(dead_code)]
pub fn part1() -> String {
    let mut safe_reports = 0;

    let reports = aoc::to_lines("input/day2.txt");
    for report in reports {
        let levels = report.split_whitespace().collect::<Vec<&str>>();
        let levels = levels.iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let mut is_increasing = true;
        let mut is_decreasing = true;
        let mut last_level = levels[0];
        for l in 1..levels.len() {
            is_increasing = is_increasing && is_ordered(last_level, levels[l]);
            is_decreasing = is_decreasing && is_ordered(levels[l], last_level);

            last_level = levels[l];
        }

        if is_increasing || is_decreasing {
            safe_reports += 1;
        }
    }

    safe_reports.to_string()
}

#[allow(dead_code)]
pub fn part2() -> String {
    let mut safe_reports = 0u32;

    let reports = aoc::to_lines("input/day2.txt");
    for report in reports {
        let levels = report.split_whitespace().collect::<Vec<&str>>();
        let levels = levels.iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let mut is_increasing = true;
        let mut last_level = levels[0];
        for l in 1..levels.len() {
            is_increasing = is_increasing && is_ordered(last_level, levels[l]);

            last_level = levels[l];
        }

        let mut is_decreasing = true;
        let mut last_level = levels[0];
        for l in 1..levels.len() {
            is_decreasing = is_decreasing && is_ordered(levels[l], last_level);

            last_level = levels[l];
        }

        if is_increasing || is_decreasing {
            safe_reports += 1;
        }
    }

    safe_reports.to_string()
}