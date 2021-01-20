pub fn part1(input : &str) -> u32 {
    let array: Vec<Vec<char>> = input
    .split("\r\n")
    .map(|s: &str| s.chars().collect())
    .collect();

    let mut tree_counter = 0;
    let mut pos: (u32, u32) = (0,0);
    let max_pos: (u32, u32) = (array[1].len()as u32 - 1, array.len() as u32 - 1);
    let iterator = (3,1);

    while pos.1 <= max_pos.1 {
        match array.get(pos.1 as usize).unwrap().get(pos.0 as usize) {
            Some(character) =>  {
                if *character == '#' {
                    tree_counter += 1;
                }
            }
            None => {
                println!("Second : {}",pos.0)
            }
        }
        if pos.0 <= max_pos.0 - iterator.0 {
            pos.0 += iterator.0;
        } else  {
            pos.0 -= max_pos.0 - iterator.0 + 1;
        }
        pos.1 += iterator.1; 
    }
    tree_counter
}

pub fn part2(input : &str) ->  u32 {
    let array: Vec<Vec<char>> = input
    .split("\r\n")
    .map(|s: &str| s.chars().collect())
    .collect();

    let mut tree_counter = 0;
    let mut pos: (u32, u32) = (0,0);
    let max_pos: (u32, u32) = (array[1].len()as u32 - 1, array.len() as u32 - 1);
    let iterators: [(u32,u32) ; 5] = [(1,1),(3,1), (5,1), (7,1), (1,2)];
    let mut values :[u32; 5] = [0; 5];

    for x in 0..5 {
        while pos.1 <= max_pos.1 {
            match array.get(pos.1 as usize).unwrap().get(pos.0 as usize) {
                Some(character) => {
                    if *character == '#' {
                        tree_counter += 1;
                    }
                }
                None => {
                    println!("Second : {}",pos.0)
                }
            }
            if pos.0 <= max_pos.0 - iterators[x].0 {
                pos.0 += iterators[x].0;
            } else  {
                pos.0 -= max_pos.0 - iterators[x].0 + 1;
            }
            pos.1 += iterators[x].1;
        }
        values[x] = tree_counter;
        tree_counter = 0;
        pos = (0,0);
    }

    let mut product = 1;
    for x in values.iter() {
        product *= x;
    } 
    product
}
