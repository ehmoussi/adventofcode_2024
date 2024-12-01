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
    let mut total_distance = 0;
    for it in left_locations.iter().zip(right_locations.iter()) {
        let (left_value, right_value) = it;
        let distance: i32 = (left_value - right_value).abs();
        // println!("{} {} : {}", left_value, right_value, distance);
        total_distance += distance;
    }
    return total_distance;
}

// the left and right locations are sorted
fn compute_similarity_score(left_locations: &Vec<i32>, right_locations: &Vec<i32>) -> i32 {
    let mut score = 0;
    for left_location in left_locations {
        let mut nb_identical = 0;
        for right_location in right_locations {
            if left_location == right_location {
                nb_identical += 1;
            } else if left_location < right_location {
                break;
            }
        }
        score += left_location * nb_identical;
    }
    return score;
}
fn main() {
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
