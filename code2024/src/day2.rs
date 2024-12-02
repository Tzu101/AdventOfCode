const MIN_ORDER_DIFF: i32 = 1;
const MAX_ORDER_DIFF: i32 = 3;
fn are_levels_ordered(prev: i32, next: i32) -> bool {
    let diff: i32 = (next - prev) as i32;
    MIN_ORDER_DIFF <= diff && diff <= MAX_ORDER_DIFF
}

fn is_report_ordered(levels: &Vec<i32>) -> bool {
    let mut is_increasing = true;
    let mut is_decreasing = true;
    let mut last_level = levels[0];
    for l in 1..levels.len() {
        is_increasing = is_increasing && are_levels_ordered(last_level, levels[l]);
        is_decreasing = is_decreasing && are_levels_ordered(levels[l], last_level);

        last_level = levels[l];
    }

    is_increasing || is_decreasing
}

fn is_report_almost_ordered(levels: &Vec<i32>) -> bool {
    let mut is_increasing = true;
    let mut last_level = levels[0];
    for l in 1..levels.len() {
        is_increasing = is_increasing && are_levels_ordered(last_level, levels[l]);

        if is_increasing {
            last_level = levels[l];
        }
        else {
            let mut potential_solution1 = levels.clone();
            potential_solution1.remove(l-1);
            let potential_solved1 = is_report_ordered(&potential_solution1);

            let mut potential_solution2 = levels.clone();
            potential_solution2.remove(l);
            let potential_solved2 = is_report_ordered(&potential_solution2);

            if potential_solved1 || potential_solved2 {
                return true;
            }
            else {
                break;
            }
        }
    }

    let mut is_decreasing = true;
    let mut last_level = levels[0];
    for l in 1..levels.len() {
        is_decreasing = is_decreasing && are_levels_ordered(levels[l], last_level);

        if is_decreasing {
            last_level = levels[l];
        }
        else {
            let mut potential_solution1 = levels.clone();
            potential_solution1.remove(l-1);
            let potential_solved1 = is_report_ordered(&potential_solution1);

            let mut potential_solution2 = levels.clone();
            potential_solution2.remove(l);
            let potential_solved2 = is_report_ordered(&potential_solution2);

            if potential_solved1 || potential_solved2 {
                return true;
            }
            else {
                break;
            }
        }
    }

    is_increasing || is_decreasing
}

#[allow(dead_code)]
pub fn part1() -> String {
    let mut safe_reports = 0;

    let reports = aoc::to_lines("input/day2.txt");
    for report in reports {
        let levels = report.split_whitespace().collect::<Vec<&str>>();
        let levels = levels.iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        if is_report_ordered(&levels) {
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
        if is_report_almost_ordered(&levels) {
            safe_reports += 1;
        }
    }

    safe_reports.to_string()
}