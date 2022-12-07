use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut curr_dir: Vec<&str> = Vec::new();
    let mut sizes: HashMap<String, u32> = HashMap::new();
    for line in contents.lines() {
        let mut parts = line.split_whitespace();
        if line.starts_with('$') {
            let _dollar = parts.next();
            let cmd = parts.next().unwrap();

            if cmd == "cd" {
                let dir = parts.next().unwrap();
                if dir == ".." {
                    curr_dir.pop();
                } else {
                    curr_dir.push(dir);
                    sizes.insert(curr_dir.join("/").replace("//", "/"), 0);
                }
            }
        } else {
            // We listening to ls output
            let dir = curr_dir.join("/").replace("//", "/");
            if line.starts_with("dir") {
                continue;
            }
            let num: u32 = parts.next().unwrap().parse().unwrap();
            *sizes.get_mut(&dir).unwrap() += num;
            let mut temp_vec: Vec<&str> = Vec::new();
            for i in 0..(curr_dir.len() - 1) {
                temp_vec.push(curr_dir.get_mut(i).unwrap());
                let parent_dir = temp_vec.join("/").replace("//", "/");
                *sizes.get_mut(&parent_dir).unwrap() += num;
            }
        }
    }

    let sum_sizes = sizes
        .iter_mut()
        .filter(|(_, size)| size < &&mut 100_000)
        .fold(0, |acc, (_, size)| acc + *size);
    println!("Part one: {}", sum_sizes);

    // Part two
    let root = *sizes.get("/").unwrap();
    let size_to_free = 30_000_000 - (70_000_000 - root);
    let mut smallest = u32::MAX;
    for (_, size) in sizes {
        if size > size_to_free && size < smallest {
            smallest = size;
        }
    }
    println!("Part two: {}", smallest);
}
