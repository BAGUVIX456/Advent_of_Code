// Advent Of Code, 2021 -> Day 3, Puzzle 2

use std::io::{BufRead, BufReader};
use std::fs::File;
use::binary_ok;

// This is a function to find the most common digit in a given position of the vector elements
fn most_common_digit (vector : &Vec<String>, pos: usize) -> &str {
    // This variable is incremented if 1 is found as the column element, else decremented
    let mut count = 0;

    for line in vector {
        if &line[pos..pos+1] == "1" {
            count += 1;
        }
        else {
            count -= 1;
        }
    }

    // If 1 is the most common element, we will get a +ve value of count, hence "1" is returned in that case
    // In the inverse case, "0" is returned
    if count >= 0 {
        "1"
    }
    else {
        "0"
    }
}

/*
  Standalone function to find the O2 generator rating and the CO2 scrubber rating from the 
  input, which I kept in a text file. The type parameter is given as 1 to get O2 generator rating, and 2 to get
  CO2 scrubber rating.
*/
fn generator_rating (gen_type: usize) -> u128 {
    let path = File::open("C:/Users/Gumbi/Desktop/Dhanvith Files/Day 3.txt").unwrap();  //this comment for you dumbass, remove file path from this sketch
    let input_file = BufReader::new(path); 

    // This vector will contain all of the input values after reading from the file
    let mut master = Vec::new(); 

    // Reads the text file line by line and pushes each line in the text file into the master vector
    for line in input_file.lines() {  
        let line = line.expect("Failed to read line");
        master.push(line);
    }

    for index in 0..12  {
        //accesses the elements in master from successive columns and finds the most common digit from each column
        let most_common = most_common_digit(&master, index);
        let mut index2 = 0;
        let mut another_required_vector = Vec::new();
        
        /* 
           If type is 1: The index of every element, whose digit in the column (just accessed above) is not the same as the most 
           common digit, is stored in another_required_vector
           If type is 2: The index of every element whose digit in the column matches with the most common digit is stored
           in another_required_vector 
        */
        for line in &master { 
            let checker = &line[index..index+1]; 
            if gen_type == 1 {
                if most_common != checker {
                    another_required_vector.push(index2);
                }
            }
            else {
                if most_common == checker {
                    another_required_vector.push(index2);
                }
            }
            index2 += 1;
        }

        let mut another_val = 0;

        // All the elements whose index is in another_required_vector is deleted from master
        for value in another_required_vector {
            master.remove(value-another_val);
            another_val += 1;
        }

        // The loop breaks when only one element is left in master
        if master.len() == 1 {
            break;
        }
    }

    // The last value is extracted, parsed to u128 and returned from the function
    let return_val = &master[0];
    let return_val: u128 = return_val.trim().parse().expect("Failed to convert string to int");
    return_val
}

fn main() {
    let o2_generator = binary_ok::u128_to_decimal(&generator_rating(1));
    let co2_scrubber = binary_ok::u128_to_decimal(&generator_rating(2));

    println!("The O2 generator rating is {}", o2_generator);
    println!("The CO2 scrubber rating is {}", co2_scrubber);
    println!();
    println!("The required product is: {}", o2_generator * co2_scrubber);
}
