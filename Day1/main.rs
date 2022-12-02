use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut highest_calories = -100;
    let mut all_calories = Vec::new();
    let mut curr_call = 0;

    let mut lines = contents.lines();
    while let Some(line) = lines.next() {
        if line == "" {
            if curr_call > highest_calories {
                highest_calories = curr_call;
            }
            all_calories.push(curr_call);
            curr_call = 0;
            continue;
        }
        curr_call += line.parse::<i32>().unwrap();
        println!("Line: {}", line);
    }

    all_calories.sort();
    let size = all_calories.len();
    let top_three = all_calories[size - 1] + all_calories[size - 2] + all_calories[size - 3];

    println!("Highest calories =  {}", highest_calories);
    println!("Top three = {}", top_three);
    //println!("With text:\n{contents}");
}
