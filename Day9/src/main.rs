use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

type Coord = (i32, i32);

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    part_one(&contents);
    let len = 10;
    let mut snake: Vec<Coord> = vec![(0, 0); len]; // Head is 0
    let mut visited_coords: HashSet<Coord> = HashSet::new();
    for line in contents.lines() {
        let motion: (&str, u32) = line
        .split_once(' ')
        .map(|f| (f.0, f.1.parse::<u32>().unwrap()))
        .unwrap();
        move_dir(&mut snake, &mut visited_coords, motion.0, motion.1, len);
        //debug_set(&visited_coords);
        //println!("-----------{}----------------", motion.0);
    }
    println!("Part two: {}", visited_coords.len());
    //debug(&visited_coords);
}

fn move_dir(snake: &mut Vec<Coord>, coords: &mut HashSet<Coord>, direction: &str, amount: u32, len: usize) {
    match direction {
        "U" => {
            for _ in 0..amount {
                snake.get_mut(0).unwrap().1 += 1;
                move_shit(snake, coords, len);
            }
        },
        "D" => {
            for _ in 0..amount {
                snake.get_mut(0).unwrap().1 -= 1;
                move_shit(snake, coords, len);
            }
        },
        "L" => {
            for _ in 0..amount {
                snake.get_mut(0).unwrap().0 -= 1;
                move_shit(snake, coords, len);
            }
        },
        "R" => {
            for _ in 0..amount {
                snake.get_mut(0).unwrap().0 += 1;
                move_shit(snake, coords, len);
            }
        }
        _ => unreachable!()
    }
    
}

fn move_shit(snake: &mut Vec<Coord>, coords: &mut HashSet<Coord>, len: usize) {
    for i in 1..len {
        let h = *snake.get_mut(i - 1).unwrap();
        let mut t = snake.get_mut(i).unwrap();
        //println!(" H({}, {}), T({}, {})", h.0, h.1, t.0, t.1);
        if is_diff1(h, *t) {
            continue;
        } else if is_diff2(h, *t) {
            println!("Diff2: H({}, {}), T({}, {})", h.0, h.1, t.0, t.1);
            if t.0 < h.0 && t.1 == h.1 {
                t.0 = h.0 - 1;
            } else if t.0 > h.0 && t.1 == h.1 {
                t.0 = h.0 + 1;
            } else if t.1 < h.1 && t.0 == h.0 {
                t.1 = h.1 - 1;
            } else if t.1 > h.1 && t.0 == h.0 {
                t.1 = h.1 + 1;
            } else {
                unreachable!();
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
    println!("Snake: {:?})", snake);
    coords.insert(last);
    //println!("--------------------------");
}

fn is_diff1(h: Coord, t: Coord) -> bool {
    let a = h.0 - t.0;
    let b = h.1 - t.1;
    let c = a*a + b*b;
    return c == 0 || c == 1 || c == 2;
}

fn is_diff2(h: Coord, t: Coord) -> bool {
    let a = h.0 - t.0;
    let b = h.1 - t.1;
    let c = a*a + b*b;
    return c == 4;
}

fn part_one(contents: &String) {
    let mut h_coord: Coord = (0, 0);
    let mut t_coord: Coord = (0, 0);
    let mut visited_coords: HashMap<Coord, u32> = HashMap::new();
    visited_coords.insert(t_coord, 1);
    for line in contents.lines() {
        let motion: (&str, u32) = line
            .split_once(' ')
            .map(|f| (f.0, f.1.parse::<u32>().unwrap()))
            .unwrap();
        match motion.0 {
            "U" => {
                for _ in 0..motion.1 {
                    h_coord.1 += 1;
                    if !is_diff1(h_coord, t_coord) {
                        t_coord.0 = h_coord.0;
                        t_coord.1 = h_coord.1 - 1;
                        if visited_coords.contains_key(&t_coord) {
                            *visited_coords.get_mut(&t_coord).unwrap() += 1;
                        } else {
                            visited_coords.insert(t_coord, 1);
                        }
                    }
                }
            },
            "D" => {
                for _ in 0..motion.1 {
                    h_coord.1 -= 1;
                    if !is_diff1(h_coord, t_coord) {
                        t_coord.0 = h_coord.0;
                        t_coord.1 = h_coord.1 + 1;
                        if visited_coords.contains_key(&t_coord) {
                            *visited_coords.get_mut(&t_coord).unwrap() += 1;
                        } else {
                            visited_coords.insert(t_coord, 1);
                        }
                    }
                }
            },
            "L" => {
                for _ in 0..motion.1 {
                    h_coord.0 -= 1;
                    if !is_diff1(h_coord, t_coord) {
                        t_coord.0 = h_coord.0 + 1;
                        t_coord.1 = h_coord.1;
                        if visited_coords.contains_key(&t_coord) {
                            *visited_coords.get_mut(&t_coord).unwrap() += 1;
                        } else {
                            visited_coords.insert(t_coord, 1);
                        }
                    }
                }
            },
            "R" => {
                for _ in 0..motion.1 {
                    h_coord.0 += 1;
                    if !is_diff1(h_coord, t_coord) {
                        t_coord.0 = h_coord.0 - 1;
                        t_coord.1 = h_coord.1;
                        if visited_coords.contains_key(&t_coord) {
                            *visited_coords.get_mut(&t_coord).unwrap() += 1;
                        } else {
                            visited_coords.insert(t_coord, 1);
                        }
                    }
                }
            },
            _ => unreachable!()
        }
        //println!("Curr t_coord: ({}, {})", t_coord.0, t_coord.1);
    }

    println!("Part one: {}", visited_coords.keys().len());
}

fn debug(map: &HashMap<Coord, u32>) {
    let max_x = map.keys().map(|c| c.0).max().unwrap();
    let max_y = map.keys().map(|c| c.1).max().unwrap();
    for y in 0..=max_y {
        for x in 0..=max_x {
            if map.contains_key(&(x, y)) {
                print!("{}", *map.get(&(x, y)).unwrap());
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn debug_set(map: &HashSet<Coord>) {
    let max_x = map.into_iter().map(|c| c.0).max().unwrap();
    let max_y = map.into_iter().map(|c| c.1).max().unwrap();
    let min_x = map.into_iter().map(|c| c.0).min().unwrap();
    let min_y = map.into_iter().map(|c| c.1).min().unwrap();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if map.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}