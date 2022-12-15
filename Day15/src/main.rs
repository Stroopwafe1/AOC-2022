mod coord;

use std::env;
use std::fs;
use std::thread;

use crate::coord::coordinates::Coord2D;

#[derive(Clone, Copy, Debug)]
struct Sensor {
    own_pos: Coord2D,
    #[allow(dead_code)]
    beac_pos: Coord2D,
    dist: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut sensors: Vec<Sensor> = get_new_sensors_list(&contents);
    // Iterate over the line coords
    // For each sensor, calculate if the current distance is smaller than the precalculated distance to its beacon
    // If it is, that position cannot contain a beacon

    // ------------ PART 1 --------------
    let row = 2_000_000;
    let iterations = 5_000_000;
    let mut impossible_beacon_count = 0;
    // for x in -iterations..iterations {
    //     let coord = Coord2D {
    //         x, y: row
    //     };
    //     let is_in_range = sensors.iter().any(|s| coord.get_manhattan_distance(s.own_pos) <= s.dist && coord != s.beac_pos);
    //     if is_in_range {
    //         impossible_beacon_count += 1;
    //     }
    // }
    // println!("Part one: {}", impossible_beacon_count);
    // ----------- END PART 1 ------------

    // Part two, find the only position where the beacon is not picked up by any sensor
    let handles = (0..14)
        .into_iter()
        .map(|i| {
            let str = contents.clone();
            thread::spawn(move || {
                let sensorss = get_new_sensors_list(&str);
                println!("Sensors: {:?}", sensorss);
                for y in 285_715 * i..285_715 * (i + 1) {
                    if y % 100_000 == 0 {
                        println!("Checking y = {}", y);
                    }
                    for x in 0..=4_000_000 {
                        let coord = Coord2D { x, y };
                        let out_of_range = sensorss
                            .iter()
                            .all(|s| coord.get_manhattan_distance(s.own_pos) > s.dist);
                        if out_of_range {
                            println!("DONE!!!!!!!!  ({}, {})", x, y);
                            println!("Part two: {}", x * 4_000_000 + y);
                            return;
                        }
                    }
                }
            })
        })
        .collect::<Vec<_>>();
    handles.into_iter().for_each(|h| h.join().unwrap());
}

fn get_new_sensors_list(str: &String) -> Vec<Sensor> {
    let mut sensors: Vec<Sensor> = Vec::new();
    for line in str.lines() {
        let (sensor, beacon) = line.split_once(':').unwrap();
        let (sensor_x, sensor_y) = sensor.split_once(',').unwrap();
        let (_, sensor_val_x) = sensor_x.split_once('=').unwrap();
        let (_, sensor_val_y) = sensor_y.split_once('=').unwrap();

        let (beacon_x, beacon_y) = beacon.split_once(',').unwrap();
        let (_, beacon_val_x) = beacon_x.split_once('=').unwrap();
        let (_, beacon_val_y) = beacon_y.split_once('=').unwrap();

        let sensor_pos = Coord2D {
            x: sensor_val_x.parse().unwrap(),
            y: sensor_val_y.parse().unwrap(),
        };
        let beacon_pos = Coord2D {
            x: beacon_val_x.parse().unwrap(),
            y: beacon_val_y.parse().unwrap(),
        };
        let sensor = Sensor {
            own_pos: sensor_pos,
            beac_pos: beacon_pos,
            dist: sensor_pos.get_manhattan_distance(beacon_pos),
        };
        sensors.push(sensor);
    }
    return sensors;
}
