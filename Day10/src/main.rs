use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    enum Instruction {
        NOOP,
        ADDX(i32),
    }

    let mut cycle_count = 1;
    let mut total_strength = 0;
    let mut x_val = 1;
    let mut crt_pos = 0;
    for line in contents.lines() {
        let split = line.split_once(' ').unwrap_or(("noop", ""));

        let instruction = match split.0 {
            "noop" => Instruction::NOOP,
            "addx" => Instruction::ADDX(split.1.parse().unwrap()),
            _ => unreachable!(),
        };

        cycle_count += 1;
        match instruction {
            Instruction::NOOP => draw(&mut crt_pos, x_val),
            Instruction::ADDX(val) => {
                cycle_count += 1;
                draw(&mut crt_pos, x_val);

                if cycle_count % 40 == 20 {
                    total_strength += x_val * cycle_count;
                }

                cycle_count += 1;
                draw(&mut crt_pos, x_val);
                x_val += val;
            }
        }

        if cycle_count % 40 == 20 {
            total_strength += x_val * cycle_count;
        }
    }

    println!("Part one: {}", total_strength);
}

fn draw(crt_pos: &mut i32, x_val: i32) {
    if *crt_pos >= x_val - 1 && *crt_pos <= x_val + 1 {
        print!("#");
    } else {
        print!(".");
    }
    *crt_pos += 1;
    if *crt_pos >= 40 {
        println!();
        *crt_pos %= 40;
    }
}
