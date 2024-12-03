use regex::Regex;
use std::{fs::File, io::Read};

fn compute(content: &str) -> i32 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut total = 0;
    for (_, [a, b]) in re.captures_iter(&content).map(|c| c.extract()) {
        let i = a.parse::<i32>().expect("failed to parse");
        let j = b.parse::<i32>().expect("failed to parse");
        total += i * j;
    }
    return total;
}

fn read_input_day3() -> String {
    let filename = "src/input_day3.txt";
    let mut file = File::open(filename).expect("failed to open the file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read the content of the file");
    return content;
}

fn part1(content: &String) -> i32 {
    return compute(&content);
}

fn part2(content: &String) -> i32 {
    let mut total = 0;
    // let content = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let mut content_tmp = String::from(content);
    loop {
        match content_tmp.split_once("don't()") {
            None => {
                total += compute(&content_tmp);
                break;
            }
            Some((left, right)) => {
                total += compute(&left);
                match right.split_once("do()") {
                    None => break,
                    Some((_, right_2)) => {
                        content_tmp = String::from(right_2);
                    }
                };
            }
        };
    }
    return total;
}

pub fn solve_day3() {
    let content = read_input_day3();
    let total_part1 = part1(&content);
    println!("total: {}", total_part1); // 187194524
    let total_part2 = part2(&content);
    println!("total: {}", total_part2); // 127092535
}
