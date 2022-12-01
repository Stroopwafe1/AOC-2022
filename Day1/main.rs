use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut highestCalories = -100;
    let mut all_calories = Vec::new();
    let mut currCall = 0;

    let mut lines = contents.lines();
    while let Some(line) = lines.next() {
        if line == "" {
            if currCall > highestCalories {
                highestCalories = currCall;
            }
            all_calories.push(currCall);
            currCall = 0;
            continue;
        }
        currCall += line.parse::<i32>().unwrap();
        println!("Line: {}", line);
    }

    all_calories.sort();
    let size = all_calories.len();
    let top_three = all_calories[size - 1] + all_calories[size - 2] + all_calories[size - 3];

    println!("Highest calories =  {}", highestCalories);
    println!("Top three = {}", top_three);
    //println!("With text:\n{contents}");
}
