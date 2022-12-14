use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Eq, Hash, PartialEq, Debug)]
struct Obstacle {
    x: i32,
    y: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut obstacles: HashSet<Obstacle> = HashSet::new();
    let mut max_y = 0;
    for line in contents.lines() {
        let coords = line.split(" -> ");
        let mut curr_coord: &str = "";
        for coord in coords {
            if curr_coord.is_empty() {
                curr_coord = coord;
                continue;
            }
            let (from_x, from_y) = curr_coord.split_once(',').map(|(s1, s2)| (s1.parse::<i32>().unwrap(), s2.parse::<i32>().unwrap())).unwrap();
            let (to_x, to_y) = coord.split_once(',').map(|(s1, s2)| (s1.parse::<i32>().unwrap(), s2.parse::<i32>().unwrap())).unwrap();
            let y_min = if from_y < to_y {from_y} else {to_y};
            let y_max = if to_y >= from_y {to_y} else {from_y};
            if y_max > max_y {
                max_y = y_max;
            }

            let x_min = if from_x < to_x {from_x} else {to_x};
            let x_max = if to_x >= from_x {to_x} else {from_x};

            for y in y_min..=y_max {
                for x in x_min..=x_max {
                    let obstacle = Obstacle {
                        x,
                        y
                    };
                    obstacles.insert(obstacle);
                }
            }
            curr_coord = coord;
        }
    }

    // Comment this for part one
    for x in -1000..=1000 {
        let obs = Obstacle {
            x,
            y: max_y + 2,
        };
        obstacles.insert(obs);
    }

    let sand_source = (500, 0);
    let max_iters = 500;
    let mut i = 0;
    let mut sand_grain = sand_source;
    let mut settled_count = 0;
    // Swap this while loop for part 1/part 2
    //while i < max_iters {
    while !impossible(&obstacles, 500, 0) {
        i += 1;
        let next_pos = (sand_grain.0, sand_grain.1 + 1);
        
        if impossible(&obstacles, next_pos.0, next_pos.1) {
            if !impossible(&obstacles, next_pos.0 - 1, next_pos.1) {
                sand_grain = (next_pos.0 - 1, next_pos.1);
                continue;
            } else {
                if !impossible(&obstacles, next_pos.0 + 1, next_pos.1) {
                    sand_grain = (next_pos.0 + 1, next_pos.1);
                    continue;
                } else {
                    // We checked down, left, and right. We can settle
                    let settle = Obstacle {
                        x: sand_grain.0,
                        y: sand_grain.1,
                    };
                    obstacles.insert(settle);
                    settled_count += 1;
                    i = 0;
                    // Spawn new grain of sand
                    sand_grain = sand_source;
                }
            }
        } else {
            sand_grain = next_pos;
        }
    }
    println!("Solution: {}", settled_count);
}

fn impossible(obstacles: &HashSet<Obstacle>, x: i32, y: i32) -> bool {
    let obs = Obstacle {
        x,
        y
    };
    obstacles.contains(&obs)
}