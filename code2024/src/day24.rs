use std::collections::{HashSet, HashMap};
use rand::Rng;

fn and(bit1: bool, bit2: bool) -> bool {
    bit1 && bit2
}

fn or(bit1: bool, bit2: bool) -> bool {
    bit1 || bit2
}

fn xor(bit1: bool, bit2: bool) -> bool {
    bit1 != bit2
}

struct Gates {
    value: Option<bool>,
    input1: String,
    input2: String,
    func: String
}

fn solve_gates(gate: &String, gates: &HashMap<String, Gates>, values: &HashMap<String, bool>) -> bool {
    if values.contains_key(gate) {
        values[gate]
    }
    else {
        let gate = &gates[gate];
        let func = if gate.func == "AND" {
            and
        }
        else {
            if gate.func == "OR" {
                or
            }
            else {
                xor
            }
        };
        func(solve_gates(&gate.input1, gates, values), solve_gates(&gate.input2, gates, values))
    }
}

#[allow(dead_code)]
pub fn part1() -> String {
    let input = aoc::to_lines("input/day24.txt");

    let mut initial_state = HashMap::new();
    let mut diagram = HashMap::new();
    let mut unsolved = HashSet::new();
    let mut inputs = HashSet::new();
    let mut diagram_input = false;
    for line in input {
        if line.is_empty() {
            diagram_input = true;
            continue;
        }

        if diagram_input {
            let line = line.split(" ").collect::<Vec<&str>>();
            let input1 = line[0].to_string();
            let input2 = line[2].to_string();
            inputs.insert(input1.clone());
            inputs.insert(input2.clone());

            let func = line[1].to_string();

            let output = line[4].to_string();
            let init_state = if initial_state.contains_key(&output) {
                Some(*initial_state.get(&output).unwrap())
            }
            else {
                unsolved.insert(output.clone());
                None
            };

            diagram.insert(output, Gates {
                value: init_state,
                input1,
                input2,
                func
            });
        }
        else {
            let line = line.split(": ").collect::<Vec<&str>>();
            initial_state.insert(line[0].to_string(), line[1].parse::<i32>().unwrap() == 1);
        }
    }

    unsolved.retain(|x| !inputs.contains(x));
    let mut sorted_outputs: Vec<i32> = unsolved
        .clone()
        .into_iter()
        .filter_map(|s| {
            let trimmed = s.chars().skip(1).collect::<String>();
            trimmed.parse::<i32>().ok()
        })
        .collect();
    sorted_outputs.sort();

    let sorted_inputs = sorted_outputs.iter().rev().collect::<Vec<&i32>>();
    let sorted_inputs = sorted_inputs.iter().map(|num| {
        format!("z{:02}", num)
    }).collect::<Vec<String>>();

    let mut output: u64 = 0;
    for input in sorted_inputs {
        output <<= 1;
        if solve_gates(&input, &diagram, &initial_state) {
            output += 1;
        }
    }

    output.to_string()
}

fn get_index(input: &str) -> usize {
    input
        .chars()
        .skip(1)
        .collect::<String>()
        .parse::<usize>().unwrap()
}

fn get_gate_id(input: &str, input_mapping: &mut HashMap<String, usize>, unique_inputs: &mut usize) -> usize {
    if input.starts_with('x') {
        get_index(input)
    }
    else if input.starts_with('y') {
        get_index(input) + INPUT_X
    }
    else if input.starts_with('z') {
        get_index(input) + INPUT_Y
    }
    else {
        if input_mapping.contains_key(input) {
            input_mapping[input]
        }
        else {
            let index = *unique_inputs;
            input_mapping.insert(input.to_string(), index);
            *unique_inputs += 1;
            index
        }
    }
}

#[derive(Debug)]
enum Operation {
    And,
    Or,
    Xor
}

const INPUT_X: usize = INPUT_SIZE;
const INPUT_Y: usize = 2 * INPUT_SIZE;
const OUTPUT_Z: usize = 2 * INPUT_SIZE + OUTPUT_SIZE;

fn has_cycle(id: usize, inputs: &HashMap<usize, (usize, usize)>, visited: &mut HashSet<usize>) -> bool {
    if visited.contains(&id) {
        return true;
    }

    if !inputs.contains_key(&id) {
        return false;
    }

    visited.insert(id);
    let (input1, input2) = inputs[&id];
    let input1_cycles = has_cycle(input1, inputs, visited);
    let input2_cycles = has_cycle(input2, inputs, visited);
    visited.remove(&id);

    input1_cycles || input2_cycles
}
fn solve_gate(id: usize, inputs: &HashMap<usize, (usize, usize)>, operations: &HashMap<usize, Operation>, input_x: &[bool], input_y: &[bool]) -> bool {
    if id < INPUT_X {
        input_x[id]
    }
    else if id < INPUT_Y {
        input_y[id - INPUT_SIZE]
    }
    else {
        let (input1, input2) = inputs[&id];
        match operations[&id] {
            Operation::And => {
                and(solve_gate(input1, inputs, operations, input_x, input_y),
                    solve_gate(input2, inputs, operations, input_x, input_y))
            }
            Operation::Or => {
                or(solve_gate(input1, inputs, operations, input_x, input_y),
                    solve_gate(input2, inputs, operations, input_x, input_y))
            }
            Operation::Xor => {
                xor(solve_gate(input1, inputs, operations, input_x, input_y),
                    solve_gate(input2, inputs, operations, input_x, input_y))
            }
        }
    }
}

const INPUT_SIZE: usize = 45;  // 6 or 45
const OUTPUT_SIZE: usize = 46;  // 6 or 46
const SWAP_NUM: usize = 4;  // 46

#[derive(Debug, Clone)]
struct Instance {
    swaps: Vec<usize>,
    score: usize
}

fn vec_to_num(inputs: &[bool]) -> usize {
    let mut result = 0;
    for input in (0..inputs.len()).rev() {
        result <<= 1;
        if inputs[input] {
            result += 1;
        }
    }
    result
}

fn calculate_result(inputs: &HashMap<usize, (usize, usize)>, operations: &HashMap<usize, Operation>, input_x: &[bool; INPUT_SIZE], input_y: &[bool; INPUT_SIZE]) -> usize {
    let mut result = 0;
    for input in (INPUT_Y..OUTPUT_Z).rev() {
        result <<= 1;
        if solve_gate(input, &inputs, &operations, input_x, input_y) {
            result += 1;
        }
    }
    result
}

#[allow(dead_code)]
pub fn part2() -> String {
    let mut input_x = [false; INPUT_SIZE];
    let mut input_y = [false; INPUT_SIZE];

    let mut inputs = HashMap::new();
    let mut operations: HashMap<usize, Operation> = HashMap::new();
    let mut viable_outputs = Vec::new();

    let mut unique_ids = 2 * INPUT_SIZE + OUTPUT_SIZE;
    let mut id_mapping = HashMap::new();

    let input_lines = aoc::to_lines("input/day24.txt");

    let mut uo = 0;
    for line in input_lines {
        let line = line.split(" ").collect::<Vec<&str>>();
        let input1 = get_gate_id(line[0], &mut id_mapping, &mut unique_ids);
        let input2 = get_gate_id(line[2], &mut id_mapping, &mut unique_ids);
        let output = get_gate_id(line[4], &mut id_mapping, &mut unique_ids);
        println!("{}({input1}) {} {}({input2}) -> {}({output})", line[0], line[1], line[2], line[4]);

        /*println!("\t{} --> i{uo}[{}]", line[0], line[1]);
        println!("\t{} --> i{uo}[{}]", line[2], line[1]);
        println!("\ti{uo} --> {}", line[4]);*/
        uo += 1;

        inputs.insert(output, (input1, input2));
        viable_outputs.push(output);

        let operation = line[1].to_string();
        let operation = if operation == "AND" {
            Operation::And
        }
        else {
            if operation == "OR" {
                Operation::Or
            }
            else {
                Operation::Xor
            }
        };
        operations.insert(output, operation);
    }

    let mut errors = 0;
    for index in 0..INPUT_SIZE-3 {
        input_x[index] = true;
        input_y[index] = true;
        input_x[index+3] = true;
        input_y[index+3] = false;

        let x = vec_to_num(&input_x);
        let y = vec_to_num(&input_y);
        let z = calculate_result(&inputs, &operations, &input_x, &input_y);

        if x+y != z {
            errors += 1;
        }
        else {
            continue;
        }

        println!("Index {index:<2}: {} + {} = {} [{}]", x, y, z, x+y == z);

        input_x[index] = false;
        input_y[index] = false;
        input_x[index+3] = false;
        input_y[index+3] = false;
    }
    println!("Found {} errors!", errors);

    /*const GENERATIONS: usize = 800;
    const GENERATION_SIZE: usize = 1024;
    const MUTATION_CHANCE: usize = 4;

    let mut instances = Vec::new();
    for _ in 0..GENERATION_SIZE {
        instances.push(random_instance(&viable_outputs, &mut inputs));
    }

    for g in 0..GENERATIONS {
        for instance in &mut instances {
            score_instance(instance, &mut inputs, &mut operations, &mut input_x, &mut input_y);
            //println!("Instance with swaps {:?} scored: {}", instance.swaps, instance.score);
        }

        instances.sort_by_key(|instance| instance.score);
        instances.reverse();
        instances.truncate(GENERATION_SIZE / 2);

        if instances[0].score == 0 {
            println!("No best instance, RESTARTING!");
            instances = Vec::new();
            for _ in 0..GENERATION_SIZE {
                instances.push(random_instance(&viable_outputs, &mut inputs));
            }
        }
        else {
            println!("Best instance of generation {g}: {:?}", instances[0]);

            for instance in &mut instances {
                instance.score = 0;
            }

            let mut p1 = 0;
            let mut p2 = 0;
            while instances.len() < GENERATION_SIZE {
                let mut new_instance = merge_instances(&instances[p1], &instances[p2], &viable_outputs, &mut inputs);

                let mut rng = rand::thread_rng();
                if rng.gen_range(0..MUTATION_CHANCE) == 0 {
                    mutate_instance(&mut new_instance, &viable_outputs, &mut inputs);
                }

                p1 += 1;
                if p1 >= GENERATION_SIZE / 4 {
                    p1 = 0;
                }

                p2 += 1;
                if p2 >= GENERATION_SIZE / 2 {
                    p2 = 0;
                }

                instances.push(new_instance);
            }
        }
    }*/

    0.to_string()
}

fn perform_swap(from: usize, to: usize, inputs: &mut HashMap<usize, (usize, usize)>) {
    let temp_from = inputs[&from];
    inputs.remove(&from);

    let temp_to = inputs[&to];
    inputs.remove(&to);

    inputs.insert(to, temp_from);
    inputs.insert(from, temp_to);
}

fn random_swap(current_swaps: &Vec<usize>, viable_swaps: &Vec<usize>, inputs: &mut HashMap<usize, (usize, usize)>) -> (usize, usize) {
    let mut rng = rand::thread_rng();

    loop {
        let swap1 = viable_swaps[rng.gen_range(0..viable_swaps.len())];
        let swap2 = viable_swaps[rng.gen_range(0..viable_swaps.len())];

        if swap1 == swap2 {
            continue;
        }

        if current_swaps.contains(&swap1) || current_swaps.contains(&swap2) {
            continue;
        }

        perform_swap(swap1, swap2, inputs);
        let cycled = has_cycle(swap1, inputs, &mut HashSet::new());
        perform_swap(swap1, swap2, inputs);
        if cycled {
            continue;
        }
        return (swap1, swap2);
    }
}

fn random_instance(viable_swaps: &Vec<usize>, inputs: &mut HashMap<usize, (usize, usize)>) -> Instance {
    let mut swaps = Vec::new();
    for _ in 0..SWAP_NUM {
        let (swap1, swap2) = random_swap(&swaps, viable_swaps, inputs);
        swaps.push(swap1);
        swaps.push(swap2);
    }

    Instance {
        swaps,
        score: 0,
    }
}

fn merge_instances(parent1: &Instance, parent2: &Instance, viable_swaps: &Vec<usize>, inputs: &mut HashMap<usize, (usize, usize)>) -> Instance {
    let mut rng = rand::thread_rng();

    let mut swaps = Vec::new();
    for s in (0..2*SWAP_NUM).step_by(2) {
        let (swap1, swap2) = if rng.gen_range(0..2) == 1 {
            (parent1.swaps[s], parent1.swaps[s + 1])
        }
        else {
            (parent2.swaps[s], parent2.swaps[s + 1])
        };

        if swaps.contains(&swap1) || swaps.contains(&swap2) {
            let (swap1, swap2) = random_swap(&swaps, viable_swaps, inputs);
            swaps.push(swap1);
            swaps.push(swap2);
        }
        else {
            swaps.push(swap1);
            swaps.push(swap2);
        }
    }

    let new_instance = Instance {
        swaps,
        score: 0
    };

    for s in (0..2*SWAP_NUM).step_by(2) {
        perform_swap(new_instance.swaps[s], new_instance.swaps[s+1], inputs);
    }

    let mut cycles = false;
    for input in (INPUT_Y..OUTPUT_Z).rev() {
        if cycles {
            break;
        }
        cycles = has_cycle(input, inputs, &mut HashSet::new());
    }

    for s in (0..2*SWAP_NUM).step_by(2) {
        perform_swap(new_instance.swaps[s], new_instance.swaps[s+1], inputs);
    }

    if !cycles {
        new_instance
    }
    else {
        random_instance(viable_swaps, inputs)
    }
}

fn mutate_instance(instance: &mut Instance, viable_swaps: &Vec<usize>, inputs: &mut HashMap<usize, (usize, usize)>) {
    let mut rng = rand::thread_rng();

    let mutate_at = rng.gen_range(0..SWAP_NUM);
    let (swap1, swap2) = random_swap(&instance.swaps, viable_swaps, inputs);
    instance.swaps[mutate_at] = swap1;
    instance.swaps[mutate_at + 1] = swap2;
}

fn score_instance(instance: &mut Instance, inputs: &mut HashMap<usize, (usize, usize)>, operations: &HashMap<usize, Operation>, input_x: &mut [bool; INPUT_SIZE], input_y: &mut [bool; INPUT_SIZE]) {
    for s in (0..2*SWAP_NUM).step_by(2) {
        perform_swap(instance.swaps[s], instance.swaps[s+1], inputs);
    }

    'nested: for o in 0..INPUT_SIZE-1 {
        let expected_result = 2 * 2_u64.pow(o as u32) + 2_u64.pow(o as u32 + 1);
        //let expected_result = 2_u64.pow(o as u32 + 1) + 1;

        input_x[o] = true;
        input_x[o + 1] = true;
        input_y[o + 1] = true;

        let mut result: u64 = 0;
        for input in (INPUT_Y..OUTPUT_Z).rev() {
            if has_cycle(input, inputs, &mut HashSet::new()) {
                instance.score = 0;
                break 'nested;
            }
            result <<= 1;
            if solve_gate(input, &inputs, &operations, input_x, input_y) {
                result += 1;
            }
        }

        input_x[o] = false;
        input_x[o + 1] = false;
        input_y[o + 1] = false;

        if expected_result == result {
            instance.score += 1;
        }
    }

    *input_x = [false; INPUT_SIZE];
    *input_y = [false; INPUT_SIZE];

    for s in (0..2*SWAP_NUM).step_by(2) {
        perform_swap(instance.swaps[s], instance.swaps[s+1], inputs);
    }
}
