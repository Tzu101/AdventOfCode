#[allow(dead_code)]
pub fn part1() -> String {
    let mut safe_reports = 0u32;

    let reports = aoc::to_lines("input/day2.txt");
    for report in reports {
        let levels = report.split_whitespace().collect::<Vec<&str>>();
        let levels = levels.iter().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();

        let mut is_increasing = true;
        let mut is_decreasing = true;
        let mut last_level = levels[0];
        let mut allow_bad_level = true;
        for l in 1..levels.len() {
            if is_increasing {
                if !(levels[l] > last_level && levels[l].abs_diff(last_level) <= 3) {
                    is_increasing = false;
                }
            }

            if is_decreasing {
                if !(levels[l] < last_level && levels[l].abs_diff(last_level) <= 3) {
                    is_decreasing = false;
                }
            }

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

    let reports = aoc::to_lines("input/day2_example.txt");
    for report in reports {
        let levels = report.split_whitespace().collect::<Vec<&str>>();
        let levels = levels.iter().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();

        let mut is_increasing = true;
        let mut is_decreasing = true;
        let mut last_level = levels[0];
        let mut allow_bad_level = true;
        for l in 1..levels.len() {
            if is_increasing {
                if !(levels[l] > last_level && levels[l].abs_diff(last_level) <= 3) {
                    is_increasing = false;
                }
            }

            if is_decreasing {
                if !(levels[l] < last_level && levels[l].abs_diff(last_level) <= 3) {
                    is_decreasing = false;
                }
            }

            last_level = levels[l];
        }

        if is_increasing || is_decreasing {
            safe_reports += 1;
        }
    }

    safe_reports.to_string()
}