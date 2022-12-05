use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let (stacks_str, instructions) = contents.split_once("\n\n").unwrap();

    let mut stacks: Vec<Vec<char>> = Vec::new();
    for line in stacks_str.lines() {
        if line.starts_with(" 1") {
            break;
        }
        let mut i = 0;
        while i < line.len() {
            let max_split = if i + 4 >= line.len() {line.len()} else {i + 4};
            let split = &line[i..max_split];
            
            if i / 4 >= stacks.len() {
                stacks.push(Vec::new());
            } 
            if !split.trim().is_empty() {
                stacks[i / 4].push(split.trim().chars().nth(1).unwrap())
            }
            i += 4;
        }
    }

    for stack in &mut stacks {
        stack.reverse();
    }

    for instruction in instructions.lines() {
        let mut nums = instruction.split_whitespace()
        .filter(|split| split.parse::<usize>().is_ok())
        .map(|s| s.parse::<usize>().unwrap());

        let amount = nums.next().unwrap();
        let source = nums.next().unwrap();
        let dest = nums.next().unwrap();

        let mut temp_stack: Vec<char> = Vec::new();
        for _ in 0..amount {
            let val = stacks[source - 1].pop().unwrap();

            // Part one
            //stacks[dest - 1].push(val);

            // Part two
            temp_stack.push(val);
        }

        // Part two as well
        while temp_stack.len() > 0 {
            let c = temp_stack.pop().unwrap();
            stacks[dest - 1].push(c);
        }
    }

    let answer: String = stacks.into_iter().map(|v| v[v.len() - 1]).collect::<Vec<char>>().iter().collect();
    println!("Answer: {:?}", answer)
}
