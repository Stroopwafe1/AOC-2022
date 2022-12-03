use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut scores = HashMap::new();
    scores.insert("A", 1);
    scores.insert("B", 2);
    scores.insert("C", 3);
    scores.insert("X", 1);
    scores.insert("Y", 2);
    scores.insert("Z", 3);

    let file_path = &args[1];
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut total_score = 0;
    for line in contents.lines() {
        let mut parts = line.split(' ');
        let p1 = parts.next().unwrap();
        let p2 = parts.next().unwrap();
        let p1_score = *scores.get(p1).unwrap();
        let p2_score = *scores.get(p2).unwrap();

        //total_score += part_one(p1_score, p2_score);
        total_score += part_two(p2, p1_score);
    }

    println!("Total score: {}", total_score)
}

fn part_one(p1_score: i32, p2_score: i32) -> i32 {
    let diff = p1_score - p2_score;
    if diff == 0 {
        // Draw
        return p2_score + 3;
    } else if diff == 1 || diff == -2 {
        // Loss
        return p2_score + 0;
    } else if diff == -1 || diff == 2 {
        // Win
        return p2_score + 6;
    }

    return 0;
}

fn part_two(p2: &str, p1_score: i32) -> i32 {
    if p2 == "Y" {
        return p1_score + 3;
    }

    for i in 1..4 {
        if p2 == "X" {
            if (p1_score - i == 1) || (p1_score - i == -2) {
                return i + 0;
            }
        } else if p2 == "Z" {
            if (p1_score - i == -1) || (p1_score - i == 2) {
                return i + 6;
            }
        }
    }
    return 0;
}