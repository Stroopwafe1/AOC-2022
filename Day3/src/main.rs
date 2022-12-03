use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut priorities = 0;
    for rucksack in contents.lines() {
        let comp1 = &rucksack[0..rucksack.len() / 2];
        let comp2 = &rucksack[rucksack.len() / 2..rucksack.len()];
        let found = comp1.chars().find(|c| comp2.contains(*c)).unwrap();
        // a-z = 1-26
        // A-Z = 27-52
        let mut score = found as i32 - 96;
        if score < 0 {
            score = found as i32 - 38;
        }
        priorities += score;
    }
    println!("Sum part one: {priorities}");
    
    // Part two 
    priorities = 0;

    let mut iter = contents.lines();
    while let Some(elf1) = iter.next() {
        let elf2 = iter.next().expect("Elf to exist");
        let elf3 = iter.next().expect("Elf to exist");
        let found = elf1.chars().find(|c| elf2.contains(*c) && elf3.contains(*c)).expect("At least one char");
        // a-z = 1-26
        // A-Z = 27-52
        let mut score = found as i32 - 96;
        if score < 0 {
            score = found as i32 - 38;
        }
        priorities += score;
    }

    println!("Sum part two: {priorities}");
}
