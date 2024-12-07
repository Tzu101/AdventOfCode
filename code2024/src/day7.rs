fn get_equation_data(equation: &str) -> (i64, Vec<i64>) {
    let equation_parts = equation.split(":").collect::<Vec<&str>>();
    let solution = equation_parts[0].parse::<i64>().unwrap();
    let mut values : Vec<i64> = Vec::new();
    for value in equation_parts[1].split_whitespace().collect::<Vec<&str>>() {
        values.push(value.parse::<i64>().unwrap());
    }

    (solution, values)
}

fn is_equation_possible(values: &[i64], current_result: i64, target_result: i64) -> bool {
    if values.is_empty() {
        return current_result == target_result;
    }

    is_equation_possible(&values[1..], current_result + values[0], target_result) ||
        is_equation_possible(&values[1..], current_result * values[0], target_result)
}

#[allow(dead_code)]
pub fn part1() -> String {
    let mut possible_equation_sum = 0;
    let equations = aoc::to_lines("input/day7.txt");
    for equation in equations {
        let (solution, values) = get_equation_data(&equation);

        if is_equation_possible(&values[1..], values[0], solution) {
            possible_equation_sum += solution;
        }
    }
    possible_equation_sum.to_string()
}

fn is_equation_possible_concat(values: &[i64], current_result: i64, target_result: i64) -> bool {
    if values.is_empty() {
        return current_result == target_result;
    }

    let is_add = is_equation_possible_concat(&values[1..], current_result + values[0], target_result);
    let is_mul = is_equation_possible_concat(&values[1..], current_result * values[0], target_result);
    let is_con = is_equation_possible_concat(&values[1..], format!("{}{}", current_result, values[0]).parse::<i64>().unwrap(), target_result);

    is_add || is_mul || is_con
}

#[allow(dead_code)]
pub fn part2() -> String {
    let mut possible_equation_sum = 0;
    let equations = aoc::to_lines("input/day7.txt");
    for equation in equations {
        let (solution, values) = get_equation_data(&equation);

        if is_equation_possible_concat(&values[1..], values[0], solution) {
            possible_equation_sum += solution;
        }
    }
    possible_equation_sum.to_string()
}
