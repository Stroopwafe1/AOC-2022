use std::env;

#[derive(Clone)]
enum OP {
    ADD(i64),
    MUL(i64),
}

#[derive(Clone)]
struct Monkey {
    items: Vec<i64>,
    op: OP,
    monke_if_false: i32,
    monke_if_true: i32,
    test_val: i64,
}

impl Monkey {
    fn inspect(&self, item: i64, part_b: i32) -> i64 {
        //println!("Monkey inspects item with level {}", item);
        let return_val;
        match self.op {
            OP::ADD(val) => return_val = item + val,
            OP::MUL(val) => {
                if val != i64::MIN {
                    return_val = item * val
                } else {
                    return_val = item * item
                }
            }
        }
        if part_b != i32::MIN {
            return return_val % part_b as i64;
        }
        //println!("Worry level is increased to {}", return_val);
        //println!("Monkey bored. Level is {}", return_val / 3);
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

fn init(to_run: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    if to_run == "example" {
        let m0 = Monkey {
            items: vec![79, 98],
            op: OP::MUL(19),
            monke_if_true: 2,
            monke_if_false: 3,
            test_val: 23,
        };
        let m1 = Monkey {
            items: vec![54, 65, 75, 74],
            op: OP::ADD(6),
            test_val: 19,
            monke_if_true: 2,
            monke_if_false: 0,
        };
        let m2 = Monkey {
            items: vec![79, 60, 97],
            op: OP::MUL(i64::MIN),
            test_val: 13,
            monke_if_true: 1,
            monke_if_false: 3,
        };
        let m3 = Monkey {
            items: vec![74],
            op: OP::ADD(3),
            test_val: 17,
            monke_if_true: 0,
            monke_if_false: 1,
        };
        monkeys.push(m0);
        monkeys.push(m1);
        monkeys.push(m2);
        monkeys.push(m3);
    } else {
        let m0 = Monkey {
            items: vec![62, 92, 50, 63, 62, 93, 73, 50],
            op: OP::MUL(7),
            test_val: 2,
            monke_if_true: 7,
            monke_if_false: 1,
        };

        let m1 = Monkey {
            items: vec![51, 97, 74, 84, 99],
            op: OP::ADD(3),
            test_val: 7,
            monke_if_true: 2,
            monke_if_false: 4,
        };

        let m2 = Monkey {
            items: vec![98, 86, 62, 76, 51, 81, 95],
            op: OP::ADD(4),
            test_val: 13,
            monke_if_true: 5,
            monke_if_false: 4,
        };

        let m3 = Monkey {
            items: vec![53, 95, 50, 85, 83, 72],
            op: OP::ADD(5),
            test_val: 19,
            monke_if_true: 6,
            monke_if_false: 0,
        };

        let m4 = Monkey {
            items: vec![59, 60, 63, 71],
            op: OP::MUL(5),
            test_val: 11,
            monke_if_true: 5,
            monke_if_false: 3,
        };
        
        let m5 = Monkey {
            items: vec![92, 65],
            op: OP::MUL(i64::MIN),
            test_val: 5,
            monke_if_true: 6,
            monke_if_false: 3,
        };

        let m6 = Monkey {
            items: vec![78],
            op: OP::ADD(8),
            test_val: 3,
            monke_if_true: 0,
            monke_if_false: 7,
        };

        let m7 = Monkey {
            items: vec![84, 93, 54],
            op: OP::ADD(1),
            test_val: 17,
            monke_if_true: 2,
            monke_if_false: 1,
        };
        monkeys.push(m0);
        monkeys.push(m1);
        monkeys.push(m2);
        monkeys.push(m3);
        monkeys.push(m4);
        monkeys.push(m5);
        monkeys.push(m6);
        monkeys.push(m7);
    }
    return monkeys;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let to_run = &args[1];
    let mut monkeys = init(to_run);
    let mut inspect_counts = vec![0; monkeys.len()];
    let part_b = monkeys.iter().fold(1, |sum, monkey| sum * monkey.test_val);
    for _ in 0..10_000 {
        let counts = monkey_business(&mut monkeys, part_b as i32);
        for i in 0..counts.len() {
            inspect_counts[i] += counts[i];
        }
    }
    inspect_counts.sort_by(|a, b| b.cmp(a));
    println!("Part one: {}", inspect_counts[0] * inspect_counts[1]);
}

fn monkey_business(monkeys: &mut Vec<Monkey>, part_b: i32) -> Vec<usize> {
    let mut inspect_counts = vec![0; monkeys.len()];
    for i in 0..monkeys.len() {
        let monkey = monkeys[i].to_owned();
        for item in monkey.items.iter() {
            let new = monkey.inspect(*item, part_b);
            let to_throw = monkey.test(new);
            inspect_counts[i] += 1;
            add_item_to_monkey(monkeys, new, to_throw as usize);
        }
        monkeys.get_mut(i).unwrap().items.clear();
    }
    //println!("Inspects: {:?}", inspect_counts);
    return inspect_counts;
}

fn add_item_to_monkey(monkeys: &mut Vec<Monkey>, item: i64, i: usize) {
    let monkey = monkeys.get_mut(i).unwrap();
    monkey.items.push(item);
}
