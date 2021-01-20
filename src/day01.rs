
pub fn part1(input : &str) -> u32{
    let array: Vec<u32> = input
        .split('\n')
        .map(|s: &str| s.trim().parse::<u32>().unwrap())
        .collect();
    for first in &array {
        for second in array.iter().skip(array.iter().position(|&x| x == *first).unwrap() + 1) {
            let total = first + second;
            if total == 2020 {
                return (first * second) as u32;
            }
        }
    }
    0
}

pub fn part2(input : &str) -> u32 {

    let array: Vec<u32> = input
        .split('\n')
        .map(|s: &str| s.trim().parse::<u32>().unwrap())
        .collect();
    
    let three_digit_nums = array.iter()
    .filter(|x| x.to_string().len() == 3)
    .collect::<Vec<&u32>>();


    let length = three_digit_nums
                .iter()
                .size_hint()
                .0;

    for first in &array {
        for y in 0..length {
            for z in (y+1)..length {
                let total = first + three_digit_nums[y] + three_digit_nums[z];
                if total == 2020 {
                    let num_2 = three_digit_nums[y];
                    let num_3 = three_digit_nums[z];
                    return first * num_2 * num_3;
                }
            }
        }
    }
    0
}