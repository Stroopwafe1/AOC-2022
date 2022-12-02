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

    let lose = 0;
    let draw = 3;
    let win = 6;

    let file_path = &args[1];
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut total_score = 0;
    for line in contents.lines() {
        let mut parts = line.split(' ');
        let p1 = parts.next().unwrap();
        let p2 = parts.next().unwrap();
        let p1_score = scores.get(p1).unwrap();
        let p2_score = scores.get(p2).unwrap();
        let mut p2_score = p2_score + 0;

        for i in 1..4 {
            if p2 == "X" {
                // Need to lose
                if (p1_score - i == 1) || (p1_score - i == -2) {
                    p2_score = i;
                }
            } else if p2 == "Y" {
                p2_score = p1_score + 0;
            } else if p2 == "Z" {
                if (p1_score - i == -1) || (p1_score - i == 2) {
                    p2_score = i;
                }
            }
            
        }
        
        
        let diff = p1_score - p2_score;
        if diff == 0 {
            // Draw
            total_score += p2_score + draw;
        }
        if diff == 1 || diff == -2 {
            // Loss
            total_score += p2_score + lose;
        }
        if diff == -1 || diff == 2 {
            // Win
            total_score += p2_score + win;
        }
    }

    println!("Total score: {}", total_score)
}
