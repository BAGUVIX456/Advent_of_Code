// Advent Of Code -> Day 4, Puzzle 1

use std::io::{BufRead, BufReader};
use std::fs::File;

#[derive(Clone)]
pub struct Bongo { 
    pub table: Vec<Vec<i32>>,
}

pub trait Operations {
    fn print(&self);
    fn play_game(&mut self) -> usize;
    fn check(&self) -> Option<usize>;
    fn check_tables(&self, checking_type: i32) -> Option<usize>;
    fn win_val(&self, table_pos: usize, cancel_value : i32) -> i32;
}

impl Operations for Vec<Bongo> {
    
    // just in case you want to print the bingo tables
    fn print(&self) {
        for tables in self {
            for row in 0..5 {
                println!("{:?}", tables.table[row]);
            }
            println!();
        }
    }
      
    fn play_game(&mut self) -> usize {
        let mut return_val = 0;
        let mut affirmation = false;
        let for_cancel = [--<replace values to be cancelled from the tables here>--];
      
        for cancel_val in for_cancel { 
            println!("cancel value is: {}", cancel_val);

            for tables in 0..self.len() {   
                 for row in 0..5 {
                    for col in 0..5 {
                        if cancel_val == self[tables].table[row][col] {                         
                            self[tables].table[row][col] = 0;
                        }
                    }
                }
            }

            affirmation = match self.check() {
                Some(val) => {
                    return_val = self.win_val(val, cancel_val);
                    true
                }
                None => false
            };
            println!("affirmation is: {}", affirmation); 
            println!();

            if affirmation { break; }
        }

        if affirmation { return_val.try_into().unwrap() }
        else{ 100 }
    }

    fn check(&self) -> Option<usize> {

        let check = match self.check_tables(1) {
            Some(val) => {
                println!("check_row returns Some({})", val);
                Some(val)
            }
            None => {
                println!("check_row returns None");
                match self.check_tables(2) {
                    Some(val) => {
                        println!("check_column returns Some({})", val);
                        Some(val)
                    }
                    None => {
                        println!("check_column returns None");
                        None
                    }
                }
            }
        };

        check
    }

    fn check_tables(&self, checking_type: i32) -> Option<usize> {
        let mut complete_status = false;
        let mut winning_table: usize = 0;
        let mut checker = 0;

        // row check
        if checking_type == 1 {

            'A: for table_pos in 0..self.len() {
                for row in 0..5 {
                    checker = 0;
                    for col in 0..5 {
                        checker += self[table_pos].table[row][col];
                    }

                    if checker == 0 {
                        winning_table = table_pos;
                        complete_status = true;
                        break 'A;
                    }
                }
            }

            if complete_status {
                return Some(winning_table);
            }
            else {
                return None;
            }
        }

        // column check
        else {

         'B:for table_pos in 0..self.len() {
                for col in 0..5 {
                    checker = 0;
                    for row in 0..5 {
                        checker += self[table_pos].table[row][col];
                    }

                    if checker == 0 {
                        winning_table = table_pos;
                        complete_status = true;
                        break 'B;
                    }
                }
            }

            if complete_status { 
                return Some(winning_table);
            }
            else {
                return None;
            }
        }
    }

    fn win_val(&self, table_pos: usize, cancel_value: i32) -> i32 {
        let mut sum = 0;

        for row in 0..5 {
            for col in 0..5 { 
                sum += self[table_pos].table[row][col]; 
            }
        }
        println!("The sum of the remaining values of table {} is {}", table_pos+1, sum);

        sum*cancel_value       
    }
}


// to merge two characters into a single i32 number
fn merge_chars (input_line: &String, pos: usize) -> i32 {
    let tens = &input_line[pos..pos+1];
    let units = &input_line[pos+1..pos+2];

    let ten : i32 = match tens.trim().parse() {
        Ok(val) => val,
        Err(_) => 0,
    };
    let unit : i32 = match units.trim().parse() {
        Ok(val) => val,
        Err(_) => 0,
    };

    let return_val = (ten*10)+unit;
    return_val
}

// to read the text file containing the input and output it as a vector of bingo tables
fn get_input() -> Vec<Bongo> {
    let file_pos = File::open("--< enter the complete file location of input txt file here >--").unwrap();
    let input_file = BufReader::new(file_pos);

    let mut gamespace: Vec<Bongo> = Vec::new();
    let mut temporary = Bongo {table: vec![vec![]]};
    temporary.table.clear();

    for line in input_file.lines() {
        let line = line.unwrap();
        let mut reader_pos = 0;
        let mut temp_vec1 = Vec::new();

        if line.is_empty() { continue; }

        while reader_pos < 14 {
            let pusher_val = merge_chars(&line, reader_pos);
            temp_vec1.push(pusher_val);

            reader_pos += 3;
        }
        temporary.table.push(temp_vec1);

        if temporary.table.len() == 5 {
            gamespace.push(temporary.clone());
            temporary.table.clear();
        }
    }
    gamespace
}

fn main() {
    let mut bingo = get_input();
    println!("The length of vector bingo is: {}", bingo.len());
    bingo.print();
    println!();
    println!("The required answer is {}", bingo.play_game());
}
