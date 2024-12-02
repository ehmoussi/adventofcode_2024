use std::fs::read_to_string;

fn parse_int(location_word: &str) -> i32 {
    return match location_word.parse::<i32>() {
        Err(e) => panic!("Can't parse the location {}: {}", location_word, e),
        Ok(location_id) => location_id,
    };
}

fn read_input_day1() -> (Vec<i32>, Vec<i32>) {
    let filename = "src/input_day1.txt";
    let mut left_locations: Vec<i32> = Vec::with_capacity(1000);
    let mut right_locations: Vec<i32> = Vec::with_capacity(1000);
    for line in read_to_string(filename).unwrap().lines() {
        let words: Vec<&str> = line.split(" ").filter(|w| w.trim() != "").collect();
        if words.len() != 2 {
            panic!("The files contains at least one line with more than two locations")
        }
        let (left_location, right_location) = (words[0], words[1]);
        left_locations.push(parse_int(left_location));
        right_locations.push(parse_int(right_location));
    }
    return (left_locations, right_locations);
}

fn compute_total_distance(left_locations: &mut Vec<i32>, right_locations: &mut Vec<i32>) -> i32 {
    return left_locations
        .iter()
        .zip(right_locations.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();
}

// the left and right locations are sorted
fn compute_similarity_score(left_locations: &Vec<i32>, right_locations: &Vec<i32>) -> i32 {
    let mut score = 0;
    let mut previous_left_location: i32 = 0;
    let mut interm_score = 0;
    let mut last_right_index = 0;
    for &left_location in left_locations {
        let mut nb_identical = 0;
        if previous_left_location != left_location {
            // println!("{}", last_right_index);
            for (right_index, &right_location) in
                right_locations.iter().enumerate().skip(last_right_index)
            {
                if left_location == right_location {
                    nb_identical += 1;
                } else if left_location < right_location {
                    last_right_index = right_index;
                    break;
                }
            }
            interm_score = left_location * nb_identical;
            previous_left_location = left_location;
        }
        score += interm_score;
    }
    return score;
}

pub fn solve_day1() {
    // let mut left_locations = vec![3, 4, 2, 1, 3, 3];
    // let mut right_locations = vec![4, 3, 5, 3, 9, 3];
    let (mut left_locations, mut right_locations) = read_input_day1();
    // sort
    left_locations.sort();
    right_locations.sort();
    // Day 1
    // - distance
    let total_distance = compute_total_distance(&mut left_locations, &mut right_locations);
    println!("The total distance is: {}", total_distance); // 3508942

    // - similarity score
    let similarity_score = compute_similarity_score(&left_locations, &right_locations);
    println!("The similarity score is: {}", similarity_score); // 26593248
}
