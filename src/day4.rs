use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn read_input_day4() -> Vec<String> {
    let filename = "src/input_day4.txt";
    let file = File::open(filename).expect("failed to open the file");
    return BufReader::new(file)
        .lines()
        .map(|line| line.expect("Can't read a line"))
        .collect();
}

fn count_from_letter(
    content: &Vec<String>,
    line: &String,
    i: usize,
    j: usize,
    searched_word: &str,
) -> i32 {
    let mut total = 0;
    // By row
    let word: String = line.chars().skip(j).take(4).collect();
    if word == searched_word {
        total += 1;
    }
    // By column
    if i < (content.len() - 3) {
        let mut word: String = String::new();
        for k in 0..4 {
            match content[i + k].chars().nth(j) {
                None => {}
                Some(cc) => {
                    word.push(cc);
                }
            };
        }
        if word == searched_word {
            total += 1;
        }
        // By diagonal - right
        if j + 3 < line.len() {
            word = String::new();
            for k in 0..4 {
                match content[i + k].chars().nth(j + k) {
                    None => {}
                    Some(cc) => {
                        word.push(cc);
                    }
                };
            }
            if word == searched_word {
                total += 1;
            }
        }
        // By diagonal - left
        if j >= 3 {
            word = String::new();
            for k in 0..4 {
                match content[i + k].chars().nth(j - k) {
                    None => {}
                    Some(cc) => {
                        word.push(cc);
                    }
                };
            }
            if word == searched_word {
                total += 1;
            }
        }
    }
    return total;
}

fn solve_part1(content: &Vec<String>) {
    let mut total = 0;
    for (i, line) in content.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let mut c_total = 0;
            if c == 'X' {
                c_total += count_from_letter(&content, line, i, j, "XMAS");
            } else if c == 'S' {
                c_total += count_from_letter(&content, line, i, j, "SAMX");
            }
            total += c_total;
        }
    }
    println!("total: {}", total); // 2464
}

fn solve_part2(content: &Vec<String>) {
    let mut total = 0;
    for (i, line) in content.iter().enumerate().take(content.len() - 1).skip(1) {
        for (j, c) in line.chars().enumerate().take(line.len() - 1).skip(1) {
            if c == 'A' {
                match (
                    content[i - 1].chars().nth(j - 1),
                    content[i - 1].chars().nth(j + 1),
                    content[i + 1].chars().nth(j - 1),
                    content[i + 1].chars().nth(j + 1),
                ) {
                    (Some(tl), Some(tr), Some(bl), Some(br)) => match (tl, tr, bl, br) {
                        ('M', 'M', 'S', 'S') => total += 1,
                        ('S', 'M', 'S', 'M') => total += 1,
                        ('S', 'S', 'M', 'M') => total += 1,
                        ('M', 'S', 'M', 'S') => total += 1,
                        _ => continue,
                    },
                    _ => continue,
                };
            }
        }
    }
    println!("total: {}", total); // 1982
}

pub fn solve_day4() {
    let _content = vec![
        String::from("MMMSXXMASM"),
        String::from("MSAMXMSMSA"),
        String::from("AMXSXMAAMM"),
        String::from("MSAMASMSMX"),
        String::from("XMASAMXAMM"),
        String::from("XXAMMXXAMA"),
        String::from("SMSMSASXSS"),
        String::from("SAXAMASAAA"),
        String::from("MAMMMXMMMM"),
        String::from("MXMXAXMASX"),
    ];
    let content = read_input_day4();
    solve_part1(&content); // 2464
    solve_part2(&content); // 1982
}
