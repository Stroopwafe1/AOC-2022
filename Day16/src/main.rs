use std::cmp::Ordering;
use std::env;
use std::fs;

#[derive(Clone, Debug)]
struct Valve {
    name: String,
    rate: i32,
    tunnel_indices: Vec<usize>,
    open: bool,
}

impl Valve {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rate.cmp(&other.rate)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut valves: Vec<Valve> = Vec::new();

    for line in contents.lines() {
        let (valve_str, tunnel_str) = line.split_once(';').unwrap();
        let mut valve_split = valve_str.split_whitespace();

        let valve_name = valve_split.nth(1).unwrap();
        let flow_str = valve_split.nth(2).unwrap();
        let rate = flow_str.split_once('=').unwrap().1.parse::<i32>().unwrap();

        let mut tunnels = tunnel_str.split_whitespace();
        tunnels.nth(3);

        let mut indices: Vec<usize> = Vec::new();
        print!("Tunnels: ");
        while let Some(tunnel) = tunnels.next() {
            print!("{}", tunnel);
            let t = Valve {
                name: tunnel.replace(",", ""),
                rate: -1,
                tunnel_indices: Vec::new(),
                open: false,
            };
            let mut index = get_index(&valves, &t);
            if index == usize::MAX {
                index = valves.len();
                valves.push(t);
            }
            indices.push(index);
        }
        println!();

        let valve = Valve {
            name: valve_name.to_owned(),
            rate,
            tunnel_indices: indices.clone(),
            open: false
        };

        let index = get_index(&valves, &valve);
        if index == usize::MAX {
            valves.push(valve);
        } else {
            let temp = valves.get_mut(index).unwrap();
            temp.rate = rate;
            for i in indices.iter() {
                temp.tunnel_indices.push(*i);
            }
        }

    }

    
    let mut sorted_valves = valves.clone();
    sorted_valves.sort_by(|a, b| b.cmp(a));
    
    println!("{:#?}", sorted_valves);

    // I don't know how to solve this problem.

}

fn get_index(valves: &Vec<Valve>, valve: &Valve) -> usize {
    for i in 0..valves.len() {
        if valves[i].name == valve.name {
            return i;
        }
    }
    return usize::MAX;
}