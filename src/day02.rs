pub fn part1(input : &str) -> u32 {
    let lines: Vec<&str> = input.split('\n').collect();
    let mut valid_pwds = 0;
    for item in &lines {
        let line:Vec<&str> = item.split(|c| vec!['-', ':', ' '].contains(&c)).collect();
        let min = line[0].parse::<u32>().unwrap();
        let max = line[1].parse::<u32>().unwrap();
        let letter_count = line[4].matches(line[2]).count() as u32;
        if min <= letter_count && max >= letter_count {
            valid_pwds += 1;
        }
    }
    valid_pwds
}

pub fn part2(input: &str) -> u32 {
    let lines: Vec<&str> = input.split('\n').collect();
    let mut valid_pwds = 0;
    for item in &lines {
        let line:Vec<&str> = item.split(|c| vec!['-', ':', ' '].contains(&c)).collect();
        let pos_1 = line[0].parse::<usize>().unwrap();
        let pos_2 = line[1].parse::<usize>().unwrap();
        let first_pos_check =  {
            line[4][pos_1 - 1..pos_1] == *line[2]
        };
        let second_pos_check = {
            line[4][pos_2 - 1..pos_2] == *line[2]
        };
        if first_pos_check ^ second_pos_check {
            valid_pwds += 1;
        }
    }
    valid_pwds
}