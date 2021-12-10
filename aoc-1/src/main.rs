use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut single_increments: i32 = 0;
    let mut window_increments: i32 = 0;
    let mut last_value = 0;
    let mut window: [i32; 4] = [0, 0, 0, 0];

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        let value: i32 = line.parse().unwrap();

        window[index%4] = value;

        let last_window_size = window[(index+1)%4] + window[(index+2)%4] + window[(index+3)%4]; 
        let this_window_size = window[(index+2)%4] + window[(index+3)%4] + window[index%4]; 

        println!("window: [{}, {}, {}, {}], index: {}, last: {}, this: {}", window[0], window[1], window[2], window[3], index, last_window_size, this_window_size);

        if index>0 && value>last_value {
            single_increments +=1;
        }
        
        if index>2 && this_window_size>last_window_size {
            window_increments +=1;
        }
        last_value = value;
    }
    println!("{} single_increments, {} window_increments.", single_increments, window_increments);
}