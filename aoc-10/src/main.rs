use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_char_points(closing_char: char) -> i64 {
    match closing_char {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

fn get_char_error(unexpected: char) -> i32 {
    match unexpected {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }

}

fn get_closing_char(opening_char: char) -> char {
    match opening_char {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => '!',
    }
}

fn calculate_error(line: &str) -> i32 {

    let mut error: i32 =0;

    let char_vec : Vec<char> = line.to_string().chars().collect();
    let mut opened_chars: Vec<char>= vec![];

    for c in char_vec {
        let opened = match c {
            '(' | '[' | '{' | '<' => true,
            _ => false,
        };  

        if opened {
            opened_chars.push(c);
        } else {
            let expected_closing_char = get_closing_char(opened_chars.pop().unwrap());
            if expected_closing_char != c {
                error = get_char_error(c);
                println!("Expected {}, got {}, error: {}", expected_closing_char, c, error);
            }
        }
    }

    return error;
}

fn calculate_line_score(line: &str) -> i64 {

    let mut line_score: i64 =0;

    let char_vec : Vec<char> = line.to_string().chars().collect();
    let mut opened_chars: Vec<char>= vec![];

    let mut unmatched: bool=false;
    for c in char_vec {
        let opened = match c {
            '(' | '[' | '{' | '<' => true,
            _ => false,
        };  

        if opened {
            opened_chars.push(c);
        } else {
            let expected_closing_char = get_closing_char(opened_chars.pop().unwrap());
            if expected_closing_char != c {
                unmatched = true;
            }
        }
    }

    // Reject error lines
    if !unmatched {
        for o in opened_chars.iter().rev() {
            let c = get_closing_char(*o);
            let p = get_char_points(c);
            line_score *= 5;
            line_score += p;
        }
    }

    return line_score;
}

fn main1() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut error_total: i32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        error_total += calculate_error(&line);
    }

    println!("Error total: {}", error_total);
}

fn main2() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut line_scores: Vec<i64> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let line_score = calculate_line_score(&line);

        if line_score > 0 {
            println!("line_score: {}", line_score);
            line_scores.push(line_score);
        }
    }

    // Sort line_scores and get middle entry
    line_scores.sort();
    let middle_index = line_scores.len()/2; // 0 base
    println!("Middle score: {}", line_scores[middle_index]);
}

fn main() {
    let part1 = false;
    if part1 {
        main1();
    } else {
        main2();
    }
}
