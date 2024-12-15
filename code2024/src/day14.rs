struct Robot {
    pos_x: i32,
    pos_y: i32,
    vel_x: i32,
    vel_y: i32,
}

#[allow(dead_code)]
pub fn part1() -> String {
    const MAP_WIDTH: i32 = 101;
    const MAP_HEIGHT: i32 = 103;
    let mut map = [[0i32; MAP_WIDTH as usize]; MAP_HEIGHT as usize];

    let robot_inputs = aoc::to_lines("input/day14.txt");
    let mut robots = Vec::<Robot>::new();

    for robot_input in robot_inputs {
        let robot_data = robot_input
            .chars()
            .filter(|c| c.is_ascii_digit() || c.is_whitespace() || *c == ',' || *c == '-')
            .collect::<String>();
        let robot_data = robot_data
            .split(|c| c == ' ' || c == ',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        robots.push(Robot {
            pos_x: robot_data[0],
            pos_y: robot_data[1],
            vel_x: robot_data[2],
            vel_y: robot_data[3]
        });
        map[robot_data[1] as usize][robot_data[0] as usize] += 1;
    }

    const DAYS: usize = 100;
    for _ in 0..DAYS {
        for robot in &mut robots {
            map[robot.pos_y as usize][robot.pos_x as usize] -= 1;

            robot.pos_x = robot.pos_x + robot.vel_x;
            if robot.pos_x < 0 {
                robot.pos_x += MAP_WIDTH;
            }
            if robot.pos_x >= MAP_WIDTH {
                robot.pos_x -= MAP_WIDTH;
            }

            robot.pos_y = robot.pos_y + robot.vel_y;
            if robot.pos_y < 0 {
                robot.pos_y += MAP_HEIGHT;
            }
            if robot.pos_y >= MAP_HEIGHT {
                robot.pos_y -= MAP_HEIGHT;
            }

            map[robot.pos_y as usize][robot.pos_x as usize] += 1;
        }
    }

    let mut top_left_robots = 0;
    for y in 0..(MAP_HEIGHT/2) {
        for x in 0..(MAP_WIDTH/2) {
            top_left_robots += map[y as usize][x as usize];
        }
    }

    let mut top_right_robots = 0;
    for y in 0..(MAP_HEIGHT/2) {
        for x in (MAP_WIDTH/2 + 1)..MAP_WIDTH {
            top_right_robots += map[y as usize][x as usize];
        }
    }

    let mut bottom_left_robots = 0;
    for y in (MAP_HEIGHT/2 + 1)..MAP_HEIGHT {
        for x in 0..(MAP_WIDTH/2) {
            bottom_left_robots += map[y as usize][x as usize];
        }
    }

    let mut bottom_right_robots = 0;
    for y in (MAP_HEIGHT/2 + 1)..MAP_HEIGHT {
        for x in (MAP_WIDTH/2 + 1)..MAP_WIDTH {
            bottom_right_robots += map[y as usize][x as usize];
        }
    }

    let safety_factor = top_left_robots * bottom_left_robots * top_right_robots * bottom_right_robots;
    safety_factor.to_string()
}

/*fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a.abs() // Ensure the result is non-negative
}

fn lcm(a: i32, b: i32) -> i32 {
    (a * b).abs() / gcd(a, b)
}*/

fn map_to_parabola(x: usize, width: usize) -> f64 {
    if width <= 1 {
        return 0.0; // Handle edge case for invalid width
    }
    let x = x as f64;
    let width = width as f64;
    4.0 * (x - 1.0) * (width - x) / ((width - 1.0).powi(2))
}

#[allow(dead_code)]
pub fn part2() -> String {
    /*let robot_inputs = aoc::to_lines("input/day14.txt");
    let mut robots = Vec::<Robot>::new();

    let mut lcm_x = 1;
    let mut lcm_y = 1;
    for robot_input in robot_inputs {
        let robot_data = robot_input
            .chars()
            .filter(|c| c.is_ascii_digit() || c.is_whitespace() || *c == ',' || *c == '-')
            .collect::<String>();
        let robot_data = robot_data
            .split(|c| c == ' ' || c == ',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        robots.push(Robot {
            pos_x: robot_data[0],
            pos_y: robot_data[1],
            vel_x: robot_data[2],
            vel_y: robot_data[3]
        });

        lcm_x = lcm(lcm_x, robot_data[2]);
        lcm_y = lcm(lcm_y, robot_data[3]);
    }
    println!("{}, {}, {:?}", lcm_x, lcm_y, lcm(lcm_x, lcm_y));

    2.to_string()*/

    const MAP_WIDTH: i32 = 101;
    const MAP_HEIGHT: i32 = 103;
    let mut map = [[0i32; MAP_WIDTH as usize]; MAP_HEIGHT as usize];

    let robot_inputs = aoc::to_lines("input/day14.txt");
    let mut robots = Vec::<Robot>::new();

    for robot_input in robot_inputs {
        let robot_data = robot_input
            .chars()
            .filter(|c| c.is_ascii_digit() || c.is_whitespace() || *c == ',' || *c == '-')
            .collect::<String>();
        let robot_data = robot_data
            .split(|c| c == ' ' || c == ',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        robots.push(Robot {
            pos_x: robot_data[0],
            pos_y: robot_data[1],
            vel_x: robot_data[2],
            vel_y: robot_data[3]
        });
        map[robot_data[1] as usize][robot_data[0] as usize] += 1;
    }

    const DAYS: usize = 10000000;
    let mut max_center_score = 0.0;
    let mut min_day = 999999999;
    for day in 0..100000 {
        for robot in &mut robots {
            map[robot.pos_y as usize][robot.pos_x as usize] -= 1;

            robot.pos_x = robot.pos_x + robot.vel_x;
            if robot.pos_x < 0 {
                robot.pos_x += MAP_WIDTH;
            }
            if robot.pos_x >= MAP_WIDTH {
                robot.pos_x -= MAP_WIDTH;
            }

            robot.pos_y = robot.pos_y + robot.vel_y;
            if robot.pos_y < 0 {
                robot.pos_y += MAP_HEIGHT;
            }
            if robot.pos_y >= MAP_HEIGHT {
                robot.pos_y -= MAP_HEIGHT;
            }

            map[robot.pos_y as usize][robot.pos_x as usize] += 1;
        }

        let mut center_score: f64 = 0.0;
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                center_score += map[y as usize][x as usize] as f64 * map_to_parabola(x as usize, MAP_WIDTH as usize) * map_to_parabola(y as usize, MAP_HEIGHT as usize);
            }
        }

        if center_score > max_center_score {
            max_center_score = center_score;
            min_day = day;

            println!();
            println!();
            for row in map {
                for num in row {
                    print!("{}", num);
                }
                println!();
            }
        }
    }

    min_day.to_string()
}