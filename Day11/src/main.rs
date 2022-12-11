use std::env;
use std::fs;

#[derive(Clone, Debug)]
enum OP {
    ADD(i64),
    MUL(i64),
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<i64>,
    op: OP,
    monke_if_false: i32,
    monke_if_true: i32,
    test_val: i64,
}

impl Monkey {
    fn inspect(&self, item: i64, part_b: i32) -> i64 {
        let return_val;
        match self.op {
            OP::ADD(val) => return_val = item + val,
            OP::MUL(val) => {
                if val != -1337 {
                    return_val = item * val
                } else {
                    return_val = item * item
                }
            }
        }
        if part_b != i32::MIN {
            return return_val % part_b as i64;
        }
        return return_val / 3;
    }

    fn test(&self, item: i64) -> i32 {
        if item % self.test_val == 0 {
            self.monke_if_true
        } else {
            self.monke_if_false
        }
    }
}

fn parse(contents: &String) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let split = contents.split("\n\n");
    for s in split {
        let mut monkey: Monkey = Monkey {
            items: Vec::new(),
            op: OP::ADD(0),
            test_val: 0,
            monke_if_true: 0,
            monke_if_false: 0
        };
        for line in s.lines() {
            if line.starts_with("Monkey") { continue };
            let (val1, val2) = line.trim().split_once(":").unwrap();
            let nums: Vec<i64> = val2.trim().split(" ").filter_map(|v| v.replace(",", "").parse::<i64>().ok()).collect();
            match val1 {
                "Starting items" => monkey.items = nums,
                "Operation" => {
                    if val2.contains('*') {
                        monkey.op = OP::MUL(*nums.first().unwrap_or(&-1337));
                    } else {
                        monkey.op = OP::ADD(nums[0]);
                    }
                },
                "Test" => monkey.test_val = nums[0],
                "If true" => monkey.monke_if_true = nums[0] as i32,
                "If false" => monkey.monke_if_false = nums[0] as i32,
                _ => unreachable!()
            }
        }
        monkeys.push(monkey);
    }
    return monkeys;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut monkeys = parse(&contents);
    let mut monkeys2 = monkeys.clone();
    let mut inspect_counts = vec![0; monkeys.len()];
    let part_b = monkeys2.iter().fold(1, |sum, monkey| sum * monkey.test_val);

    for _ in 0..20 {
        let counts = monkey_business(&mut monkeys, i32::MIN);
        for i in 0..counts.len() {
            inspect_counts[i] += counts[i];
        }
    }
    inspect_counts.sort_by(|a, b| b.cmp(a));
    println!("Part one: {}", inspect_counts[0] * inspect_counts[1]);

    inspect_counts = vec![0; monkeys.len()];

    for _ in 0..10_000 {
        let counts = monkey_business(&mut monkeys2, part_b as i32);
        for i in 0..counts.len() {
            inspect_counts[i] += counts[i];
        }
    }
    inspect_counts.sort_by(|a, b| b.cmp(a));
    println!("Part two: {}", inspect_counts[0] * inspect_counts[1]);
}

fn monkey_business(monkeys: &mut Vec<Monkey>, part_b: i32) -> Vec<usize> {
    let mut inspect_counts = vec![0; monkeys.len()];
    for i in 0..monkeys.len() {
        let monkey = monkeys[i].to_owned();
        for item in monkey.items.iter() {
            let new = monkey.inspect(*item, part_b);
            let to_throw = monkey.test(new);
            inspect_counts[i] += 1;
            {
                let monkey = monkeys.get_mut(to_throw as usize).unwrap();
                monkey.items.push(new);
            }
        }
        monkeys.get_mut(i).unwrap().items.clear();
    }
    return inspect_counts;
}
