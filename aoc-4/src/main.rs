use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Lines;

fn calculate_unmarked_sum(bingo_card: &Vec<i32>) -> i32 {
    let mut unmarked_sum: i32 = 0;
    for i in bingo_card {
        if i != &-1 {
            unmarked_sum += i;
        }
    }
    return unmarked_sum;
}


/*
  Check all cards for bingo
*/

fn check_bingo_cards(bingo_cards: &Vec<Vec<i32>>, winning_order: &mut Vec<i32>) {
    for (index, bingo_card) in bingo_cards.iter().enumerate() {
        let winning_card = index as i32;

        if !winning_order.contains(&winning_card) {
            if check_for_bingo(bingo_card) {
                winning_order.push(winning_card);
            }
        }
    }
}
    
/*
  Check for bingo on a single card
*/

// Bingo card is vector of 25 numbers
// 10 ways to win:
//
// Rows:  0, 1, 2, 3, 4  Columns: 0,5,10,15,20
//        5, 6, 7, 8, 9           1,6,11,16,21
//       10,11,12,13,14           2,7,12,17,22
//       15,16,17,18,19           3,8,13,18,23
//       20,21,22,23,24           4,9,14,19,24

fn check_for_bingo(b: &Vec<i32>) -> bool {
    let mut bingo: bool = false;

    if b[0]+b[1]+b[2]+b[3]+b[4] == -5 {bingo = true;}
    else if b[5]+b[6]+b[7]+b[8]+b[9] == -5 {bingo = true;}
    else if b[10]+b[11]+b[12]+b[13]+b[14] == -5 {bingo = true;}
    else if b[15]+b[16]+b[17]+b[18]+b[19] == -5 {bingo = true;}
    else if b[20]+b[21]+b[22]+b[23]+b[24] == -5 {bingo = true;}
    else if b[0]+b[5]+b[10]+b[15]+b[20] == -5 {bingo = true;}
    else if b[1]+b[6]+b[11]+b[16]+b[21] == -5 {bingo = true;}
    else if b[2]+b[7]+b[12]+b[17]+b[22] == -5 {bingo = true;}
    else if b[3]+b[8]+b[13]+b[18]+b[23] == -5 {bingo = true;}
    else if b[4]+b[9]+b[14]+b[19]+b[24] == -5 {bingo = true;}

    return bingo;
}

/*
  Cross of matches to drawn number
  on all bingo cards (by setting value to -1)
*/

fn cross_off_all(draw: i32, winning_order: &Vec<i32>, bingo_cards: &mut Vec<Vec<i32>>) {
    let mut index: i32 = 0;
    for bingo_card in bingo_cards {
        if !winning_order.contains(&index) {
            cross_off(draw, bingo_card);
        }
        index += 1;
    }
}

/*
  Cross of matches to drawn number
  on a single bingo card (by setting value to -1)
*/

fn cross_off(draw: i32, bingo_card: &mut Vec<i32>) {
    for number in bingo_card.iter_mut() {
        if *number == draw {
            *number = -1;
        }
    }
}

/*
  Read a single bingo card
  from buffered lines iterator
*/
fn read_bingo<B: BufRead>(lines: &mut Lines<B>) -> Vec<i32> {

    let mut bingo_card: Vec<i32> = vec![];

    for _i in 0..5 {
        let line = lines.next();
        if line.is_some() {
            let line = line.unwrap().unwrap();
            let mut row: Vec<i32> = line.split(" ").filter(|&x| !x.is_empty()).map(|x| x.parse::<i32>().unwrap()).collect();
            bingo_card.append(&mut row);
        }
    }
    lines.next(); // blank line after bingo card

    return bingo_card;
}

// Bingo card is vector of 25 numbers
// 10 ways to win:
//
// Rows:  0, 1, 2, 3, 4  Columns: 0,5,10,15,20
//        5, 6, 7, 8, 9           1,6,11,16,21
//       10,11,12,13,14           2,7,12,17,22
//       15,16,17,18,19           3,8,13,18,23
//       20,21,22,23,24           4,9,14,19,24

fn main() {
    let filename = "input_test.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // First line is number sequence
    let first_line = lines.next().unwrap().unwrap();
    let numbers_to_draw: Vec<i32> = first_line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    lines.next();
    println!("Number of draws: {} ", numbers_to_draw.len());

    // Now read the bingo cards
    let mut bingo_cards: Vec<Vec<i32>> = vec![];
    loop {
        let bingo_card = read_bingo(&mut lines);
        if bingo_card.len() == 25 {
            bingo_cards.push(bingo_card);
        } else {
            break;
        }
    }

    println!("Number of bingo cards: {} ", bingo_cards.len());

    // Iterate over the draw numbers and cross them off on the bingo cards
    let mut winning_order: Vec<i32> = vec![];
    let mut winning_numbers: Vec<i32> = vec![];
    
    for draw in numbers_to_draw {
       cross_off_all(draw, &winning_order, &mut bingo_cards);

       check_bingo_cards(&bingo_cards, &mut winning_order);

       while winning_order.len() > winning_numbers.len() {
           winning_numbers.push(draw);
       }
    }

    let winning_card = winning_order[0];
    let losing_card = *winning_order.last().unwrap();
    let losing_number = winning_numbers.last().unwrap();

    for (index, bingo_card) in bingo_cards.iter().enumerate() {
        let card_number = index as i32;
        if card_number == winning_card {
            let unmarked_sum = calculate_unmarked_sum(bingo_card);
            let squid_number = unmarked_sum * winning_numbers[0];
        
            println!("Winning card: {}, winning number: {}, unmarked_sum: {}, squid_number: {}", winning_card, winning_numbers[0], unmarked_sum, squid_number);
        } else if card_number == losing_card {
            let unmarked_sum = calculate_unmarked_sum(bingo_card);
            let squid_number = unmarked_sum * losing_number;
        
            println!("Losing card: {}, Losing number: {}, unmarked_sum: {}, squid_number: {}", losing_card, losing_number, unmarked_sum, squid_number);
        }
    }
}
