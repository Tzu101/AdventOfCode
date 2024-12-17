#[derive(Debug)]
struct Register {
    a: u64,
    b: u64,
    c: u64,
}

fn to_combo(value: u64, register: &Register) -> u64 {
    if value <= 3 {
        return value;
    }
    else if value == 4 {
        return register.a;
    }
    else if value == 5 {
        return register.b;
    }
    else if value == 6 {
        return register.c;
    }

    panic!("Invalid combo value: {}", value);
}

fn adv(value: u64, register: &mut Register, _output: &mut Option<String>, _ip: &mut usize) -> bool {
    let result = register.a / 2_u64.pow(to_combo(value, &register) as u32);
    register.a = result;
    false
}

fn bxl(value: u64, register: &mut Register, _output: &mut Option<String>, _ip: &mut usize) -> bool {
    let result = value ^ register.b;
    register.b = result;
    false
}

fn bst(value: u64, register: &mut Register, _output: &mut Option<String>, _ip: &mut usize) -> bool {
    let result = to_combo(value, &register) % 8;
    register.b = result;
    false
}

fn jnz(value: u64, register: &mut Register, _output: &mut Option<String>, ip: &mut usize) -> bool {
    if register.a == 0 {
        return false;
    }

    *ip = value as usize;
    true
}

fn bxc(_value: u64, register: &mut Register, _output: &mut Option<String>, _ip: &mut usize) -> bool {
    let result = register.b ^ register.c;
    register.b = result;
    false
}

fn out(value: u64, register: &mut Register, output: &mut Option<String>, _ip: &mut usize) -> bool {
    let result = to_combo(value, &register) % 8;

    if let Some(output) = output {
        *output += &format!(",{result}");
    }
    else {
        *output = Some(result.to_string());
    }

    false
}

fn bdv(value: u64, register: &mut Register, _output: &mut Option<String>, _ip: &mut usize) -> bool {
    let result = register.a / 2_u64.pow(to_combo(value, &register) as u32);
    register.b = result;
    false
}

fn cdv(value: u64, register: &mut Register, _output: &mut Option<String>, _ip: &mut usize) -> bool {
    let result = register.a / 2_u64.pow(to_combo(value, &register) as u32);
    register.c = result;
    false
}

fn simulate_program(instructions: &Vec<char>, mut register: Register) -> String {
    let mut instruction_pointer = 0;
    let mut output: Option<String> = None;
    let instruction_list = vec![adv, bxl, bst, jnz, bxc, out, bdv, cdv];

    while instruction_pointer < instructions.len() {
        let instruction = instructions[instruction_pointer].to_digit(10).unwrap() as usize;
        let value = instructions[instruction_pointer + 1].to_digit(10).unwrap() as u64;

        if instruction_list[instruction](value, &mut register, &mut output, &mut instruction_pointer) {
            continue;
        }
        instruction_pointer += 2;
    }

    output.unwrap()
}

#[allow(dead_code)]
pub fn part1() -> String {
    let mut register = Register { a: 0, b: 0, c: 0 };
    let mut instructions = Vec::<char>::new();

    let input = aoc::to_string("input/day17_example.txt");
    let input = input.split("\n").collect::<Vec<&str>>();
    for i in 0..input.len() {
        match i {
            0 => {
                register.a = input[i].chars().filter(|c| c.is_numeric()).collect::<String>().parse::<u64>().unwrap();
            },
            1 => {
                register.b = input[i].chars().filter(|c| c.is_numeric()).collect::<String>().parse::<u64>().unwrap();
            },
            2 => {
                register.c = input[i].chars().filter(|c| c.is_numeric()).collect::<String>().parse::<u64>().unwrap();
            },
            4 => {
                instructions = input[i].chars().filter(|c| c.is_numeric() || c == &',').collect::<String>().split(',').collect::<String>().chars().collect::<Vec<char>>();
            },
            _ => continue,
        }
    }

    simulate_program(&instructions, register)
}

#[allow(dead_code)]
pub fn part2() -> String {
    let mut instructions = Vec::<char>::new();
    let mut instruction_hash: String = String::new();

    let input = aoc::to_string("input/day17.txt");
    let input = input.split("\n").collect::<Vec<&str>>();
    for i in 0..input.len() {
        match i {
            4 => {
                instruction_hash = input[i].chars().filter(|c| c.is_numeric() || c == &',').collect::<String>();
                instructions = instruction_hash.split(',').collect::<String>().chars().collect::<Vec<char>>();
            },
            _ => continue,
        }
    }

    instruction_hash = instruction_hash.replace(',', "");
    let mut reg_a = 0;
    for pos in (0..instructions.len()).rev() {
        reg_a <<= 3;
        loop {
            let result = simulate_program(&instructions, Register { a: reg_a, b: 0, c: 0 }).replace(',', "");
            println!("{}  {}", result, instruction_hash[pos..].to_string());
            if result == instruction_hash[pos..].to_string() {
                break;
            }
            reg_a += 1;
        }
    }

    reg_a.to_string()
}
