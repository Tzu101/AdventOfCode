fn find_next_empty(blocks: &Vec::<Option<u64>>, start: usize) -> usize {
    for i in (start+1)..blocks.len() {
        if blocks[i].is_none() {
            return i;
        }
    }
    blocks.len() - 1
}

fn find_next_value(blocks: &Vec::<Option<u64>>, start: usize) -> usize {
    for i in (0..start).rev() {
        if blocks[i].is_some() {
            return i;
        }
    }
    0
}

#[allow(dead_code)]
pub fn part1() -> String {
    let disk = aoc::to_char("input/day9.txt");
    let disk = &disk[0];
    //println!("{:?}", disk);

    let mut blocks = Vec::<Option<u64>>::new();
    let mut first_empty = None;
    for f in 0..disk.len() {
        let block_size = disk[f].to_digit(10).unwrap() as usize;
        let mut block_id = None;
        if f % 2 == 0 {
            block_id = Some((f / 2) as u64);
        }
        else if first_empty.is_none() {
            first_empty = Some(blocks.len());
            //println!("Empty: {first_empty:?}");
        }

        for _ in 0..block_size {
            blocks.push(block_id);
        }
    }
    //println!("{:?}", blocks.iter().map(|b| if b.is_some() { b.unwrap().to_string() } else { String::from(".") }).collect::<Vec<String>>());

    let mut move_to = first_empty.unwrap();
    let mut move_from = blocks.len() - 1;
    while move_to < move_from {
        //println!("Move {:?} from {} to {}", blocks[move_from], move_from, move_to);
        let temp_block = blocks[move_to];
        blocks[move_to] = blocks[move_from];
        blocks[move_from] = temp_block;

        move_to = find_next_empty(&blocks, move_to);
        move_from = find_next_value(&blocks, move_from);
        //println!("{:?}", blocks.iter().map(|b| if b.is_some() { b.unwrap().to_string() } else { String::from(".") }).collect::<Vec<String>>());
    }

    let mut checksum: u64 = 0;
    for b in 0..blocks.len() {
        if let Some(block) = blocks.get(b).unwrap() {
            checksum += block * b as u64;
        }
    }
    checksum.to_string()
}

#[allow(dead_code)]
pub fn part2() -> String {
    String::from("2")
}