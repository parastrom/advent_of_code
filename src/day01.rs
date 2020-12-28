
pub fn part1(input : &String) -> u32{
    let array: Vec<u32> = input
        .trim()
        .split('\n')
        .map(|s: &str| s.parse::<u32>().unwrap())
        .collect();
    for x in 0..array.len() {
        let first = array[x];
        for y in (x+1)..array.len(){
            let second = array[y];
            let total = first + second;
            if total == 2020 {
                return first * second;
            }
        }
    }
    0
}

 pub fn part2(input : &String) -> u32 {
    let array: Vec<&str> = input.split("\n").collect();
    let mut three_digit_nums: Vec<u32> = Vec::new();

    for x in 0..array.len() {
        let number = array[x];
        let temp = number.to_string();
        let length = temp.chars().count();
        if 3 == length {
            three_digit_nums.push(array[x].parse().unwrap());
        } 
    }

    let length = three_digit_nums
                 .iter()
                 .size_hint()
                 .0;

    for x in 0..array.len() {
         for y in 0..length {
             for z in (y+1)..length {
                 let total = array[x].parse::<u32>().unwrap() + three_digit_nums[y] + three_digit_nums[z];
                 if total == 2020 {
                     let num_1:u32 = array[x].parse().unwrap();
                     let num_2 = three_digit_nums[y];
                     let num_3 = three_digit_nums[z];
                     return num_1 * num_2 * num_3;
                 }
             }
         }
     }
     0
 }