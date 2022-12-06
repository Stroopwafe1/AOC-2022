use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    for i in 1..=contents.len() {
        if i + 3 > contents.len() {
            break;
        }
        let c1 = contents.chars().nth(i - 1).unwrap();
        let c2 = contents.chars().nth(i).unwrap();
        let c3 = contents.chars().nth(i + 1).unwrap();
        let c4 = contents.chars().nth(i + 2).unwrap();
        if c1 != c2 && c1 != c3 && c1 != c4 && c2 != c3 && c2 != c4 && c3 != c4 {
            println!("First packet at {}", i + 3);
            break;
        }
    }

    for i in 1..=contents.len() {
        if i + 14 > contents.len() {
            break;
        }
        let str = &contents[i - 1..i + 13];
        let mut vec = Vec::from_iter(str.chars());
        vec.sort();
        vec.dedup();
        
        if  vec.len() == str.len() {
            println!("First message packet at {}, {}", i + 13, str);
            break;
        }
    }
}
