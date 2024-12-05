use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn read_input_day5() -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let filename = "src/input_day5.txt";
    let file = File::open(filename).expect("failed to open the file");
    let mut ordering_rules = HashMap::with_capacity(49);
    let mut page_updates = Vec::with_capacity(202);
    for line in BufReader::new(file)
        .lines()
        .map(|l| l.expect("Failed to read a line"))
    {
        if line.contains('|') {
            match line.split_once("|") {
                None => {}
                Some((a, b)) => {
                    let key = a.parse::<i32>().expect("failed to parse to i32");
                    let value = b.parse::<i32>().expect("failed to parse to i32");
                    match ordering_rules.get_mut(&key) {
                        None => {
                            ordering_rules.insert(key, vec![value]);
                        }
                        Some(values) => {
                            values.push(value);
                        }
                    };
                    // println!("{:?}", ordering_rules.last().unwrap());
                }
            }
        } else if line.contains(',') {
            let update: Vec<i32> = line
                .split(',')
                .map(|p| p.parse::<i32>().expect("Failed to parse to i32"))
                .collect();
            page_updates.push(update);
            // println!("{:?}", page_updates.last().unwrap());
        }
    }
    return (ordering_rules, page_updates);
}

fn check_ordered(ordering_rules: &HashMap<i32, Vec<i32>>, update: &Vec<i32>) -> bool {
    let mut is_ordered = true;
    for (j, page) in update.iter().take(update.len() - 1).enumerate() {
        if let Some(after_pages) = ordering_rules.get(page) {
            if !after_pages.contains(&update[j + 1]) {
                is_ordered = false;
                // println!("{} not in {:?}", update[i + 1], after_pages);
                break;
            }
        } else {
            // println!("{} not in {:?}", page, _ordering_rules.keys());
            is_ordered = false;
            break;
        }
    }
    return is_ordered;
}

fn solve(ordering_rules: &HashMap<i32, Vec<i32>>, page_updates: &Vec<Vec<i32>>) -> (i32, i32) {
    let mut total_ordered = 0;
    let mut total_unordered = 0;
    for update in page_updates {
        let is_ordered = check_ordered(ordering_rules, update);
        if is_ordered {
            total_ordered += update[update.len() / 2];
        } else {
            let mut ordered_update = update.to_vec();
            ordered_update.sort_by(|page_1, page_2| {
                match ordering_rules.get(page_1) {
                    None => return Ordering::Greater,
                    Some(after_pages) => {
                        if after_pages.contains(page_2) {
                            return Ordering::Less;
                        } else {
                            return Ordering::Greater;
                        }
                    }
                };
            });
            total_unordered += ordered_update[ordered_update.len() / 2];
        }
    }
    return (total_ordered, total_unordered);
}

pub fn solve_day5() {
    let _ordering_rules = HashMap::from([
        (47, vec![53, 61, 13, 29]),
        (97, vec![13, 61, 47, 29, 53, 75]),
        (75, vec![61, 13, 29, 53, 47]),
        (61, vec![13, 53, 29]),
        (53, vec![29, 13]),
        (29, vec![13]),
    ]);
    let _page_updates = vec![
        vec![75, 47, 61, 53, 29],
        vec![97, 61, 53, 29, 13],
        vec![75, 29, 13],
        vec![75, 97, 47, 61, 53],
        vec![61, 13, 29],
        vec![97, 13, 75, 29, 47],
    ];
    let (ordering_rules, page_updates) = read_input_day5();
    let (total_ordered, total_unordered) = solve(&ordering_rules, &page_updates);
    println!("total ordered: {}", total_ordered); // 7074
    println!("total unordered: {}", total_unordered); // 4828
}
