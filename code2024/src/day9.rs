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

#[derive(Clone)]
#[derive(Debug)]
enum Block {
    Empty(usize),
    File(usize, u64),
}

fn find_suitable_empty_block(blocks: &Vec::<Block>, size: usize) -> Option<usize> {
    for i in 0..blocks.len() {
        if let Block::Empty(empty_size) = blocks.get(i).unwrap() {
            if *empty_size >= size {
                return Some(i);
            }
        }
    }
    None
}

fn find_next_file_block(blocks: &Vec::<Block>, start: usize) -> usize {
    for i in (0..start).rev() {
        if let Block::File(_, _) = blocks.get(i).unwrap() {
            return i;
        }
    }
    0
}

#[allow(dead_code)]
pub fn part2() -> String {
    let disk = aoc::to_char("input/day9.txt");
    let disk = &disk[0];
    //println!("{:?}", disk);

    let mut blocks = Vec::<Block>::new();
    for f in 0..disk.len() {
        let block_size = disk[f].to_digit(10).unwrap() as usize;
        let mut block_id = Block::Empty(block_size);
        if f % 2 == 0 {
            block_id = Block::File(block_size, (f / 2) as u64);
        }

        blocks.push(block_id);
    }
    /*println!("{:?}", blocks.iter().map(|b| {
        return match b {
            Block::Empty(size) => {
                format!("Empty({})", size)
            }
            Block::File(size, id) => {
                format!("File({size}, {id})")
            }
        }
    }).collect::<Vec<String>>());*/

    let mut move_from = blocks.len() - 1;
    while move_from > 0 {
        let file_size: usize;
        let file_block = &blocks[move_from];
        if let Block::File(file_block_size, ..) = file_block {
            file_size = file_block_size.clone();
        }
        else {
            panic!("File block is not a file");
        }

        let move_to = find_suitable_empty_block(&blocks, file_size);
        if move_to.is_none() {
            move_from = find_next_file_block(&blocks, move_from);
            continue;
        }
        let move_to = move_to.unwrap();
        if (move_to >= move_from) {
            move_from = find_next_file_block(&blocks, move_from);
            continue;
        }

        let empty_size: usize;
        let empty_block = &blocks[move_to];
        if let Block::Empty(empty_block_size) = empty_block {
            empty_size = empty_block_size.clone();
        }
        else {
            panic!("Empty block is not empty");
        }

        println!("Move {:?} from {} to {}", file_block, move_from, move_to);
        if file_size == empty_size {
            let temp_block = file_block.clone();
            blocks[move_from] = empty_block.clone();
            blocks[move_to] = temp_block;
            println!("MOVING");
        }
        else if file_size < empty_size {
            let temp_block = file_block.clone();
            blocks[move_from] = Block::Empty(file_size);
            blocks[move_to] = temp_block;
            blocks.insert(move_to + 1, Block::Empty(empty_size - file_size));
            println!("FRAGMENTING");
        }
        move_from = find_next_file_block(&blocks, move_from);

        /*println!("{:?}", blocks.iter().map(|b| {
            return match b {
                Block::Empty(size) => {
                    format!("Empty({})", size)
                }
                Block::File(size, id) => {
                    format!("File({size}, {id})")
                }
            }
        }).collect::<Vec<String>>());*/
    }

    let mut checksum: u64 = 0;
    let mut total_ids: u64 = 0;
    for block in blocks {
        match block {
            Block::Empty(size) => {
                total_ids += size as u64;
            }
            Block::File(size, id) => {
                for _ in 0..size {
                    checksum += total_ids * id;
                    total_ids += 1;
                }
            }
        }
    }
    checksum.to_string()
}