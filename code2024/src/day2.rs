const MIN_ORDER_DIFF: i32 = 1;
const MAX_ORDER_DIFF: i32 = 3;
fn are_levels_ordered(prev: i32, next: i32) -> bool {
    let diff: i32 = (next - prev) as i32;
    MIN_ORDER_DIFF <= diff && diff <= MAX_ORDER_DIFF
}

fn is_report_ordered(levels: &Vec<i32>) -> bool {
    let mut is_ordered = true;
    let mut last_level = levels[0];
    for l in 1..levels.len() {
        is_ordered = is_ordered && are_levels_ordered(last_level, levels[l]);
        last_level = levels[l];
    }
    is_ordered
}

fn is_report_safe(levels: &mut Vec<i32>) -> bool {
    is_report_ordered(levels) || is_report_ordered({ levels.reverse(); &levels })
}

fn is_dampened_report_ordered(levels: &mut Vec<i32>) -> bool {
    let mut is_almost_ordered = true;
    let mut last_level = levels[0];
    for l in 1..levels.len() {
        is_almost_ordered = is_almost_ordered && are_levels_ordered(last_level, levels[l]);

        if is_almost_ordered {
            last_level = levels[l];
        }
        else {
            let mut potential_solution1 = levels.clone();
            potential_solution1.remove(l-1);
            let potential_solved1 = is_report_ordered(&mut potential_solution1);

            let mut potential_solution2 = levels.clone();
            potential_solution2.remove(l);
            let potential_solved2 = is_report_ordered(&mut potential_solution2);

            if potential_solved1 || potential_solved2 {
                return true;
            }
            else {
                break;
            }
        }
    }
    is_almost_ordered
}

fn is_dampened_report_safe(levels: &mut Vec<i32>) -> bool {
    is_dampened_report_ordered(levels) || is_dampened_report_ordered({ levels.reverse(); levels })
}

#[allow(dead_code)]
pub fn part1() -> String {
    let mut safe_reports = 0;

    let reports = aoc::to_lines("input/day2.txt");
    for report in reports {
        let levels = report.split_whitespace().collect::<Vec<&str>>();
        let mut levels = levels.iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        if is_report_safe(&mut levels) {
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
        let mut levels = levels.iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        if is_dampened_report_safe(&mut levels) {
            safe_reports += 1;
        }
    }

    safe_reports.to_string()
}