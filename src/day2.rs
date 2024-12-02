use crate::day1::parse_int;
use std::fs::read_to_string;

fn read_input_day2() -> Vec<Vec<i32>> {
    let filename: &str = "src/input_day2.txt";
    let mut reports: Vec<Vec<i32>> = Vec::with_capacity(1000);
    for line in read_to_string(filename).unwrap().lines() {
        let words: Vec<&str> = line.split(" ").filter(|w| w.trim() != "").collect();
        let mut report: Vec<i32> = Vec::with_capacity(8);
        for word in words {
            report.push(parse_int(word));
        }
        reports.push(report);
    }
    return reports;
}

fn check_report(report: &Vec<i32>) -> bool {
    let is_increasing = report[0] <= report[1];
    for (i, &level) in report.iter().enumerate().skip(1) {
        let diff = level - report[i - 1];
        if (is_increasing && (diff < 1 || diff > 3)) || (!is_increasing && (-diff < 1 || -diff > 3))
        {
            return false;
        };
    }
    return true;
}

// TODO: Brute force :/
fn check_report_dampener(report: &Vec<i32>) -> bool {
    let is_increasing = report[0] <= report[1];
    for (i, &level) in report.iter().enumerate().skip(1) {
        let diff = level - report[i - 1];
        if (is_increasing && (diff < 1 || diff > 3)) || (!is_increasing && (-diff < 1 || -diff > 3))
        {
            for (j, _) in report.iter().enumerate() {
                let damped_report = Vec::from_iter(
                    report
                        .iter()
                        .enumerate()
                        .filter(|&(k, _)| k != j)
                        .map(|(_, l)| l.clone()),
                );
                let is_safe = check_report(&damped_report);
                if is_safe {
                    return is_safe;
                }
            }
            return false;
        };
    }
    return true;
}

pub fn solve_day2() {
    let _reports = vec![
        vec![7, 6, 4, 2, 1],
        vec![1, 2, 7, 8, 9],
        vec![9, 7, 6, 2, 1],
        vec![1, 3, 2, 4, 5],
        vec![8, 6, 4, 4, 1],
        vec![1, 3, 6, 7, 9],
    ];
    let mut reports = read_input_day2();
    let mut nb_safe = 0;
    for report in &reports {
        let is_safe = check_report(report);
        // println!("{}", is_safe);
        nb_safe += is_safe as i32;
    }
    println!("The number of safe levels is: {}", nb_safe); // 572
    let mut nb_safe_dampener = 0;
    for report in &mut reports {
        let is_safe = check_report_dampener(report);
        // println!("{:?}: {}", report, is_safe);
        nb_safe_dampener += is_safe as i32;
    }
    println!(
        "The number of safe levels (dampener) is: {}",
        nb_safe_dampener
    ); // 612
}
