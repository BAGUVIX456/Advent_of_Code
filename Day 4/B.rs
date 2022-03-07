use std::{io::{BufReader, BufRead}, fs::File};

#[derive(Clone)]
struct Bongo {
    table : Vec<Vec<i32>>,
}

pub trait Operations {
    fn print(&self);
    fn play_game(&mut self) -> i32;
    fn check_tables(&self, checking_type : i32) -> Option<Vec<usize>>;
    fn win_val(&self, cancel: i32) -> i32; 
}

impl Operations for Vec<Bongo> {

    // just in case you want to print the bingo tables
    fn print(&self) {
        for tables in self {
            for row in &tables.table {
                println!("{:?}", row);
            }
            println!();
        }
    }

    fn play_game(&mut self) -> i32 {
        let mut return_val = 0;
        let for_cancel = [--< Enter values to be cancelled into this array >--];

        'b: for cancel_val in for_cancel {
            
            for table_pos in 0..self.len() {
             'a:for row in 0..5 {
                    for column in 0..5 {
                        if self[table_pos].table[row][column] == cancel_val {
                            self[table_pos].table[row][column] = 0;
                            break 'a;
                        }
                    }
                }
            }

            let mut counter = 0;

            if let Some(remove_vals) = self.check_tables(1) {
                for value in remove_vals {

                    if self.len() == 1 {
                        return_val = self.win_val(cancel_val);
                        break 'b;
                    }

                    self.remove(value-counter);
                    counter += 1;
                }
            }

            else if let Some(remove_vals) = self.check_tables(2) {
                for value in remove_vals {

                    if self.len() == 1 {
                        return_val = self.win_val(cancel_val);
                        break 'b;
                    }

                    self.remove(value-counter);
                    counter += 1;
                }
            }
        }
        return_val
    }

    fn check_tables(&self, checking_type: i32) -> Option<Vec<usize>> {
        let mut finished : Vec<usize> = Vec::new();

        // row check
        if checking_type == 1 {

            for table_pos in 0..self.len() {
                'a: for row in 0..5 {
                        let mut sum = 0;
                        for column in 0..5 {
                            sum += self[table_pos].table[row][column];
                        }

                        if sum == 0 {
                            finished.push(table_pos);
                            break 'a;
                        }
                    }
            }

            if finished.len() == 0 {
                println!("No table finished yet");
                return None;
            }
            else {
                println!("{:?} row complete", finished);
                return Some(finished);
            }
        }
        
        // column check
        else {

            for table_pos in 0..self.len() {
                'b: for column in 0..5 {
                        let mut sum = 0;
                        for row in 0..5 {
                            sum += self[table_pos].table[row][column];
                        }

                        if sum == 0 {
                            finished.push(table_pos);
                            break 'b;
                        }
                    }
            }

            if finished.len() == 0 {
                return None;
            }
            else {
                return Some(finished);
            }
        }
    }

    fn win_val(&self, cancel: i32) -> i32 {
        let mut sum = 0;
        for row in 0..5 {
            for column in 0..5 {
                sum += self[0].table[row][column];
            }
        }

        sum*cancel
    }
}

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

fn get_input() -> Vec<Bongo> {
    let file_pos = File::open("--< enter the location of the text file which contains all the bingo tables to be input >--").unwrap();
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
            temp_vec1.push(merge_chars(&line, reader_pos));

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
    bingo.print();
    let answer = bingo.play_game();
    println!();
    println!("The required value is {}", answer);
}
