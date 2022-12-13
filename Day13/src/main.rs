use std::cmp::Ordering;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let pair_parts = contents.split("\n\n");
    let mut index = 1;
    let mut index_sum = 0;
    for pairs in pair_parts {
        let (left, right) = pairs.split_once('\n').unwrap();
        println!("== Pair {} ==", index);
        let comp = cmp(left, right, 0, true);
        match comp {
            Ordering::Less => index_sum += index,
            Ordering::Equal => panic!("Something went wrong!"),
            Ordering::Greater => {},
        }
        println!();
        index += 1;
    }
    println!("Part one: {}", index_sum);

    let mut all_lines: Vec<&str> = contents.lines().filter(|l| !l.trim().is_empty()).collect();
    all_lines.push("[[2]]");
    all_lines.push("[[6]]");
    all_lines.sort_by(|a, b| cmp(a, b, 0, false));
    
    let first = find_index(&all_lines, "[[2]]") + 1;
    let last =find_index(&all_lines, "[[6]]") + 1;
    println!("Part two: {}", first * last);
}

fn find_index(strings: &Vec<&str>, needle: &str) -> usize {
    for i in 0..strings.len() {
        let s = strings[i];
        if s == needle {
            return i;
        }
    }
    return 0;
}

fn cmp(left: &str, right: &str, depth: u8, debug: bool) -> Ordering { // -1 for left smaller, 0 for equal, 1 for right smaller
    if debug {
        println!("{:indent$}- Compare {} vs {}", "", left, right, indent = (depth * 4) as usize);
    }
    if left.starts_with('[') && right.starts_with('[') {
        let left_items = get_top_level_items(left);
        let right_items = get_top_level_items(right);

        let mut check_index = 0;
        let mut comp = Ordering::Equal;
        while comp == Ordering::Equal {
            let l_item = left_items.get(check_index).unwrap_or(&String::new()).to_owned();
            let r_item = right_items.get(check_index).unwrap_or(&String::new()).to_owned();
            if l_item.is_empty() && r_item.is_empty() {
                return Ordering::Equal;
            }
            comp = cmp(&l_item, &r_item, depth + 1, debug);
            check_index += 1;
        }
        return comp;
    } else if left.is_empty() && !right.is_empty() {
        if debug {
            println!("{:indent$}- Left side ran out of items, so inputs are in the right order", "", indent = (depth * 4) as usize);
        }
        return Ordering::Less;
    } else if !left.is_empty() && right.is_empty() {
        if debug {
            println!("{:indent$}- Right side ran out of items, so inputs are not in the right order", "", indent = (depth * 4) as usize);
        }
        return Ordering::Greater;
    } 
    else if left.starts_with('[') && !right.starts_with('[') {
        // Left is list, right is a number
        let num: u8 = right.parse().unwrap();
        if debug {
            println!("{:indent$}- Mixed types; convert right to [{}] and retry comparison", "", num, indent = (depth * 4) as usize);
        }
        return cmp(left, format!("[{}]", num).as_str(), depth, debug);
    } else if !left.starts_with('[') && right.starts_with('[') {
        let num: u8 = left.parse().unwrap();
        if debug {
            println!("{:indent$}- Mixed types; convert left to [{}] and retry comparison", "", num, indent = (depth * 4) as usize);
        }
        return cmp(format!("[{}]", num).as_str(), right, depth, debug);
    } else {
        let l_num: u8 = left.parse().unwrap();
        let r_num: u8 = right.parse().unwrap();
        let comp = l_num.cmp(&r_num);
        if debug {
            match comp {
                Ordering::Less => println!("{:indent$}- Left side is smaller, so inputs are in the right order", "", indent = (depth * 4) as usize),
                Ordering::Equal => {},
                Ordering::Greater => println!("{:indent$}- Right side is smaller, so inputs are not in the right order", "", indent = (depth * 4) as usize),
            }
        }
        return comp;
    }
}

fn get_top_level_items(input: &str) -> Vec<String> {
    // [[0, 1], [2, 3]] => { '[0, 1]', '[2, 3]'}
    let mut stack: Vec<char> = Vec::new();
    let mut return_value: Vec<String> = Vec::new();
    let mut temp: Vec<char> = Vec::new();
    for c in input.chars() {
        match c {
            '[' => {
                if stack.len() != 0 {
                    temp.push('[');
                }
                stack.push('[');
            }
            ']' => {
                stack.pop();
                if stack.len() != 0 {
                    temp.push(']');
                }
            }
            ',' => {
                if stack.len() == 1 {
                    return_value.push(temp.iter().collect());
                    temp.clear();
                } else {
                    temp.push(',');
                }
            }
            extra => temp.push(extra),
        }
    }
    return_value.push(temp.iter().collect());
    return return_value;
}