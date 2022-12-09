mod coord;

use std::collections::HashSet;
use std::env;
use std::fs;

use crate::coord::coordinates::Coord2D;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let len = 10;
    let mut snake1: Vec<Coord2D> = vec![Coord2D::new(0, 0); 2]; // Head is 0
    let mut snake2: Vec<Coord2D> = vec![Coord2D::new(0, 0); len]; // Head is 0
    let mut visited_coords1: HashSet<Coord2D> = HashSet::new();
    let mut visited_coords2: HashSet<Coord2D> = HashSet::new();

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

fn move_dir(
    snake: &mut Vec<Coord2D>,
    coords: &mut HashSet<Coord2D>,
    direction: &str,
    amount: u32,
    len: usize,
) {
    for _ in 0..amount {
        let head_movement = match direction {
            "U" => Coord2D { x:  0, y:  1 },
            "D" => Coord2D { x:  0, y: -1 },
            "L" => Coord2D { x: -1, y:  0 },
            "R" => Coord2D { x:  1, y:  0 },
            _ => unreachable!(),
        };
        *snake.get_mut(0).unwrap() += head_movement;
        move_snake(snake, coords, len);
    }
}

fn move_snake(snake: &mut Vec<Coord2D>, coords: &mut HashSet<Coord2D>, len: usize) {
    for i in 1..len {
        let h = *snake.get_mut(i - 1).unwrap();
        let t = snake.get_mut(i).unwrap();
        let c = h.get_distance(*t);
        let diff = h - *t;

        if c <= 2 {
            // Can only be 0, 1, 2
            continue;
        } else if c == 4 {
            // One space difference
            *t = h + -diff.normalise();
        } else {
            // Move diagonally
            *t += diff.normalise();
        }
    }
    let last = *snake.last_mut().unwrap();
    coords.insert(last);
}
