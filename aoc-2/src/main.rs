use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut forward :i32 = 0;
    let mut down :i32 = 0;
    
    let mut aim :i32 = 0;
    let mut depth :i32 = 0;

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let fields: Vec<&str> = line.split(' ').collect();

        let value: i32 = fields[1].parse().unwrap();

        if fields[0] == "forward" {
            forward += value;
            depth += value * aim;
        } else if fields[0] == "down" {
            down += value;
            aim += value;
        } else if fields[0] == "up" {
            down -= value;
            aim -= value;
        }
        else {
            println!("UNHANDLED: {}: forward={}, down={}", line, forward, down);
        }

        println!("{}: {}: forward={}, down={}, aim={}, depth={}", index, line, forward, down, aim, depth);
    }

    println!("forward: {}, down: {}, aim: {}, depth: {}, total_1: {}, total_2: {}", forward, down, aim, depth, forward*down, forward*depth);
}