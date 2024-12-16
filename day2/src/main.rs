use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_file(path: &str) -> io::Result<String> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut contents = String::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                contents.push_str(&format!("{}", line));
                contents.push('\n');
            }
            Err(e) => return Err(e),
        }
    }
    Ok(contents)
}

fn part_one(contents: &str) {
    let mut num_safe = 0;
    for line in contents.split('\n') {
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();

        if nums.len() < 1 {
            break;
        }
        if check_level_safe(&nums) {
            num_safe += 1;
        }
    }
    println!("num of safe reports: {}", num_safe);
}

fn check_level_safe(nums: &Vec<i32>) -> bool {
    let mut prev_num = nums[0];
    let mut is_safe = true;
    let is_asending = (nums[0] - nums[1]) < 0;
    let mut first_iter = true;
    for num in nums.iter() {
        if !first_iter && (prev_num == *num) {
            is_safe = false;
            break;
        }
        first_iter = false;
        if (prev_num - num).abs() > 3 {
            is_safe = false;
            break;
        }

        if is_asending && (*num < prev_num) {
            is_safe = false;
            break;
        }
        if !is_asending && (*num > prev_num) {
            is_safe = false;
            break;
        }
        prev_num = *num;
    }

    is_safe
}

fn part_two(contents: &str) {
    let mut num_safe = 0;
    for line in contents.split('\n') {
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();

        if nums.len() < 1 {
            break;
        }
        if check_level_safe(&nums) {
            num_safe += 1;
        } else {
            for i in 0..nums.len() {
                let mut modified_nums = nums.clone();
                modified_nums.remove(i);
                if check_level_safe(&modified_nums) {
                    num_safe += 1;
                    break;
                }
            }
        }
    }
    println!("part 2: num safe levels: {}", num_safe)
}

fn main() {
    let file_path = "input.txt";
    let contents = read_file(file_path);
    match contents {
        Ok(contents) => {
            part_one(&contents);
            part_two(&contents);
        }
        Err(e) => panic!("{}", e.to_string()),
    }
}
