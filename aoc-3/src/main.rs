use std::fs::File;
use std::io::{BufRead, BufReader};

fn main1() {
    let filename = "input_test.txt";
    let bits = 5;
//    let filename = "input.txt";
//    let bits = 12;

    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut bit_counts_basic: Vec<i32> = vec![0; bits]; // +1 for 1, -1 for 0

    for line in reader.lines() {
        let line = line.unwrap();

        for (index, c) in line.chars().enumerate() {
            bit_counts_basic[index] += if c=='0' {-1} else {1};
        }
    }

    let mut epsilon: i32 = 0;
    let mut gamma: i32 = 0;

    for bit_count in bit_counts_basic {

        // Make room for next bit
        epsilon <<= 1;
        gamma <<= 1;

        if bit_count>0 {
            epsilon += 1
        } else if bit_count<0 {
            gamma += 1;
        }
    }

    println!("Epsilon: {}, Gamma: {}, Power: {}.", epsilon, gamma, epsilon*gamma);
}

fn main2() {
//    let filename = "input_test.txt";
//    let bits = 5;
    let filename = "input.txt";
    let bits = 12;

    let mut oxygen_generator: i32 = 0;
    let mut c02_scrubber: i32 = 0;

    let mut oxygen_filter = ' ';
    let mut c02_filter = ' ';

    let mut oxygen_lines_removed: Vec<i32> = vec![0; 1000];
    let mut c02_lines_removed: Vec<i32> = vec![0; 1000];

    for bit_index in 0..bits {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let mut bit_count_oxygen: i32 = 0;
        let mut bit_count_c02: i32 = 0;
        let mut hit_c02_zero = false;
        let mut hit_c02_one = false;
    
        for (line_number, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            let line_chars: Vec<char> = line.chars().collect();
            let c = line_chars[bit_index];
            let c_previous = if bit_index > 0 {line_chars[bit_index-1]} else {' '};
            
            if oxygen_lines_removed[line_number]==0 && c_previous == oxygen_filter {
                bit_count_oxygen += if c=='0' {-1} else {1};
            } else {
                oxygen_lines_removed[line_number] = 1;
            }
            
            if c02_lines_removed[line_number]==0 && c_previous == c02_filter {
                if c=='0' {
                    hit_c02_zero = true;
                    bit_count_c02 -= 1;
                } else {
                    hit_c02_one = true;
                    bit_count_c02 += 1;
                }
            } else {
                c02_lines_removed[line_number] = 1;
            }
        }

        let oxygen_bit = if bit_count_oxygen <0 {0} else {1};
        let c02_bit = 
            if hit_c02_zero && hit_c02_one {
                if bit_count_c02 >=0 {0} else {1}
            } else if hit_c02_zero {0} else {1};

        println!("c02_filter: {}, bit_count_c02: {}, c02_bit: {}", c02_filter, bit_count_c02, c02_bit);

        oxygen_filter = if oxygen_bit==1 {'1'} else {'0'};
        c02_filter = if c02_bit==1 {'1'} else {'0'};

        oxygen_generator <<= 1;
        oxygen_generator += oxygen_bit;

        c02_scrubber <<= 1;
        c02_scrubber += c02_bit;
    }

    println!("Oxygen: {}, C02: {}, Life Support: {}.", oxygen_generator, c02_scrubber, oxygen_generator*c02_scrubber);
}
    
fn main() {
    let part1 = false;
    if part1 {
        main1();
    } else {
        main2();
    }
}
