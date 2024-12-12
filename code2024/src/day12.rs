use std::collections::HashSet;
const NO_PLANT: char = '#';

fn find_region(origin: (usize, usize), garden: &mut Vec<Vec<char>>, plot: &mut Vec<Option<(usize, usize)>>) {
    let plant = garden[origin.0][origin.1];
    garden[origin.0][origin.1] = NO_PLANT;
    plot[origin.0 * garden[origin.0].len() + origin.1] = Some(origin);

    if origin.0 > 0 && plant == garden[origin.0 - 1][origin.1] {
        find_region((origin.0 - 1, origin.1), garden, plot);
    }
    if origin.0 < garden.len() - 1 && plant == garden[origin.0 + 1][origin.1] {
        find_region((origin.0 + 1, origin.1), garden, plot);
    }
    if origin.1 > 0 && plant == garden[origin.0][origin.1 - 1] {
        find_region((origin.0, origin.1 - 1), garden, plot);
    }
    if origin.1 < garden[origin.0].len() - 1 && plant == garden[origin.0][origin.1 + 1] {
        find_region((origin.0, origin.1 + 1), garden, plot);
    }
}

fn plot_area(plot: &Vec<Option<(usize, usize)>>) -> u32 {
    plot.iter().filter(|&x| x.is_some()).count() as u32
}

fn plot_perimeter(plot: &Vec<Option<(usize, usize)>>, height: usize, width: usize) -> u32 {
    let mut perimeter = 0;
    for plant in plot {
        if let Some((row, col)) = plant {
            if *row == 0 {
                perimeter += 1;
            }
            else {
                if plot[(row - 1) * width + col].is_none() {
                    perimeter += 1;
                }
            }

            if *row == height - 1 {
                perimeter += 1;
            }
            else {
                if plot[(row + 1) * width + col].is_none() {
                    perimeter += 1;
                }
            }

            if *col == 0 {
                perimeter += 1;
            }
            else {
                if plot[row * width + (col - 1)].is_none() {
                    perimeter += 1;
                }
            }

            if *col == width - 1 {
                perimeter += 1;
            }
            else {
                if plot[row * width + (col + 1)].is_none() {
                    perimeter += 1;
                }
            }
        }
    }
    perimeter
}

#[allow(dead_code)]
pub fn part1() -> String {
    let mut garden = aoc::to_char("input/day12_example.txt");
    let mut plots = Vec::<Vec<Option<(usize, usize)>>>::new();

    for row in 0..garden.len() {
        for col in 0..garden[row].len() {
            if garden[row][col] == NO_PLANT {
                continue;
            }

            let mut plot: Vec<Option<(usize, usize)>> = vec![None; garden.len() * garden[row].len()];
            find_region((row, col), &mut garden , &mut plot);
            plots.push(plot);
        }
    }

    let mut cost = 0;
    for plot in plots {
        let area = plot_area(&plot);
        let perimeter = plot_perimeter(&plot, garden.len(), garden[0].len());

        cost += area * perimeter;
        println!("Area: {area}, Perimeter: {perimeter}");
    }

    cost.to_string()
}

fn find_region_cheap(origin: (usize, usize), garden: &mut Vec<Vec<char>>, plot: &mut Vec<(usize, usize)>) {
    let plant = garden[origin.0][origin.1];
    garden[origin.0][origin.1] = NO_PLANT;
    plot.push(origin);

    if origin.0 > 0 && plant == garden[origin.0 - 1][origin.1] {
        find_region_cheap((origin.0 - 1, origin.1), garden, plot);
    }
    if origin.0 < garden.len() - 1 && plant == garden[origin.0 + 1][origin.1] {
        find_region_cheap((origin.0 + 1, origin.1), garden, plot);
    }
    if origin.1 > 0 && plant == garden[origin.0][origin.1 - 1] {
        find_region_cheap((origin.0, origin.1 - 1), garden, plot);
    }
    if origin.1 < garden[origin.0].len() - 1 && plant == garden[origin.0][origin.1 + 1] {
        find_region_cheap((origin.0, origin.1 + 1), garden, plot);
    }
}

fn plot_perimeter_cheap(plot: &Vec<(usize, usize)>, height: usize, width: usize) -> u32 {
    let mut edges = 0;
    for plant in plot {
        let go_left = plant.1 - 1;
        let go_right = plant.1 + 1;
        let go_up = plant.0 - 1;
        let go_down = plant.0 + 1;

        let left_air = !plot.contains(&(plant.0, go_left));
        let right_air = !plot.contains(&(plant.0, go_right));
        let up_air = !plot.contains(&(go_up, plant.1));
        let down_air = !plot.contains(&(go_down, plant.1));

        if left_air == up_air && !plot.contains(&(go_up, go_left)) {
            edges += 1;
        }
        else if left_air && up_air && plot.contains(&(go_up, go_left)) {
            edges += 1;
        }

        if left_air == down_air && !plot.contains(&(go_down, go_left)) {
            edges += 1;
        }
        else if left_air && down_air && plot.contains(&(go_down, go_left)) {
            edges += 1;
        }

        if right_air == up_air && !plot.contains(&(go_up, go_right)) {
            edges += 1;
        }
        else if right_air && up_air && plot.contains(&(go_up, go_right)) {
            edges += 1;
        }

        if right_air == down_air && !plot.contains(&(go_down, go_right)) {
            edges += 1;
        }
        else if right_air && down_air && plot.contains(&(go_down, go_right)) {
            edges += 1;
        }
    }
    edges
}

#[allow(dead_code)]
pub fn part2() -> String {
    let mut garden = aoc::to_char("input/day12.txt");
    let mut plots = Vec::<(Vec<(usize, usize)>)>::new();

    for row in 0..garden.len() {
        for col in 0..garden[row].len() {
            if garden[row][col] == NO_PLANT {
                continue;
            }

            let mut plot = Vec::<(usize, usize)>::new();
            find_region_cheap((row, col), &mut garden , &mut plot);
            plots.push((plot));
        }
    }

    let mut cost = 0;
    for plot in plots {
        let area = plot.len() as u32;
        let perimeter = plot_perimeter_cheap(&plot, garden.len(), garden[0].len());

        cost += area * perimeter;
        println!("Area: {area}, Perimeter: {perimeter}");
    }

    cost.to_string()
}