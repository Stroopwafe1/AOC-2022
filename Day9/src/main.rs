use std::collections::HashSet;
use std::env;
use std::fs;

type Coord = (i32, i32);

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let len = 10;
    let mut snake1: Vec<Coord> = vec![(0, 0); 2]; // Head is 0
    let mut snake2: Vec<Coord> = vec![(0, 0); len]; // Head is 0
    let mut visited_coords1: HashSet<Coord> = HashSet::new();
    let mut visited_coords2: HashSet<Coord> = HashSet::new();

    for line in contents.lines() {
        let motion: (&str, u32) = line
        .split_once(' ')
        .map(|f| (f.0, f.1.parse::<u32>().unwrap()))
        .unwrap();
        move_dir(&mut snake1, &mut visited_coords1, motion.0, motion.1, 2);
        move_dir(&mut snake2, &mut visited_coords2, motion.0, motion.1, len);
    }
    println!("Part one: {}", visited_coords1.len());
    println!("Part two: {}", visited_coords2.len());
}

fn move_dir(snake: &mut Vec<Coord>, coords: &mut HashSet<Coord>, direction: &str, amount: u32, len: usize) {
    for _ in 0..amount {
        match direction {
            "U" => snake.get_mut(0).unwrap().1 += 1,
            "D" => snake.get_mut(0).unwrap().1 -= 1,
            "L" => snake.get_mut(0).unwrap().0 -= 1,
            "R" => snake.get_mut(0).unwrap().0 += 1,
            _ => unreachable!()
        }
        move_snake(snake, coords, len);
    }
}

fn move_snake(snake: &mut Vec<Coord>, coords: &mut HashSet<Coord>, len: usize) {
    for i in 1..len {
        let h = *snake.get_mut(i - 1).unwrap();
        let mut t = snake.get_mut(i).unwrap();
        let a = h.0 - t.0;
        let b = h.1 - t.1;
        let c = a*a + b*b;

        if c <= 2 { // Can only be 0, 1, 2
            continue;
        } else if c == 4 { // One space difference
            if a == 0 {
                t.1 = if b == -2 { h.1 + 1 } else { h.1 - 1 };
            } else {
                t.0 = if a == -2 { h.0 + 1 } else { h.0 - 1 };
            }
        } else {
            // Move diagonally
            if t.1 < h.1 {
                t.1 += 1;
            } else if t.1 > h.1 {
                t.1 -= 1;
            }
            if t.0 < h.0 {
                t.0 += 1;
            } else if t.0 > h.0 {
                t.0 -= 1;
            }
        }
    }
    let last = *snake.last_mut().unwrap();
    coords.insert(last);
}