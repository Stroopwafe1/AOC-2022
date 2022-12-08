use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let char_arr: Vec<char> = contents.chars().filter(|c| *c != '\n').collect();
    let mut i:usize = 0;
    let mut visible_trees = 0;
    let mut highest_score = i32::MIN;
    for line in contents.lines() {
        for _ in line.chars() {
            let visible = check_if_visible(&char_arr, line.len(), i);
            let score = get_scenic_score(&char_arr, line.len(), i);
            if visible {
                visible_trees += 1;
            }
            if score > highest_score {
                highest_score = score;
            }
            i += 1;
        }
    }

    println!("Part one: {}", visible_trees);
    println!("Part two: {}", highest_score);
}

fn check_if_visible(arr: &Vec<char>, line_len: usize, i: usize) -> bool {
    let row = i / line_len;
    let col = i % line_len;
    let row_max = arr.len() / line_len - 1;
    let col_max = line_len - 1;
    let char = *arr.get(i).unwrap();
    
    if row == 0 || row == row_max || col == 0 || col == col_max {
        return true;
    }
    
    // if visible in row
    let mut row_up_visible = true;
    for r in 1..=row {
        if arr.get(i - r * line_len).unwrap() >= &char {
            row_up_visible = false;
        }
    }
    let mut row_down_visible = true;
    for r in 1..=(row_max - row) {
        if arr.get(i + r * line_len).unwrap() >= &char {
            row_down_visible = false;
        }
    }

    let mut col_left_visible = true;
    for c in 1..=col {
        if arr.get(i - c).unwrap() >= &char {
            col_left_visible = false;
        }
    }
    let mut col_right_visible = true;
    for c in 1..=(col_max - col) {
        if arr.get(i + c).unwrap() >= &char {
            col_right_visible = false;
        }
    }
    let result = row_up_visible || row_down_visible || col_left_visible || col_right_visible;
    if result {
        //println!("Row: {row}, Col: {col}, char: {char}, i: {i}, up: {row_up_visible}, down: {row_down_visible}, left: {col_left_visible}, right: {col_right_visible}");
    }

    return result;
}

fn get_scenic_score(arr: &Vec<char>, line_len: usize, i: usize) -> i32 {
    let row = i / line_len;
    let col = i % line_len;
    let row_max = arr.len() / line_len - 1;
    let col_max = line_len - 1;
    let char = *arr.get(i).unwrap();
    
    let mut up_score = 0;
    for r in 1..=row {
        up_score += 1;
        if arr.get(i - r * line_len).unwrap() >= &char {
            break;
        }
    }
    let mut down_score = 0;
    for r in 1..=(row_max - row) {
        down_score += 1;
        if arr.get(i + r * line_len).unwrap() >= &char {
            break;
        }
    }

    let mut left_score = 0;
    for c in 1..=col {
        left_score += 1;
        if arr.get(i - c).unwrap() >= &char {
            break;
        }
    }

    let mut right_score = 0;
    for c in 1..=(col_max - col) {
        right_score += 1;
        if arr.get(i + c).unwrap() >= &char {
            break;
        }
    }

    return up_score * down_score * left_score * right_score;
}