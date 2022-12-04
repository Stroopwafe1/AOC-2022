use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut overlaps = 0;
    let mut overlaps_p2 = 0;
    for pair in contents.lines() {
        let mut pair_split = pair.split(',');
        let mut elf1_split = pair_split.next().expect("Have value").split('-');
        let mut elf2_split = pair_split.next().expect("Have value").split('-');

        let elf1_range = elf1_split.next().expect("value").parse::<i32>().unwrap()..(elf1_split.next().expect("value").parse::<i32>().unwrap() + 1);
        let elf2_range = elf2_split.next().expect("value").parse::<i32>().unwrap()..(elf2_split.next().expect("value").parse::<i32>().unwrap() + 1);
        
        let elf1: HashSet<i32> = HashSet::from_iter(elf1_range);
        let elf2: HashSet<i32> = HashSet::from_iter(elf2_range);

        if (elf1.is_subset(&elf2) || elf2.is_subset(&elf1)) && !elf1.is_empty() && !elf2.is_empty() {
            overlaps += 1;
        }

        if !elf1.is_disjoint(&elf2) {
            overlaps_p2 += 1;
        }
    }

    println!("Part one overlaps: {overlaps}");
    println!("Part two overlaps: {overlaps_p2}");
}
