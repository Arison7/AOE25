use std::fs::{read_to_string, File};
use std::io::prelude::*;





pub fn crack_passord() -> std::io::Result<()> {
    let mut count = 0;
    let mut dial = 50;
    let path = "./input_files/day1.txt";
    let mut addition = true;
    let mut number : i32 = 1;
    let mut starting_dial;
    
    // read file line by line 
    for line in read_to_string(path).unwrap().lines() {
        // Iterate over chars
        for (i,c) in line.chars().enumerate(){ 
            // Match index with operation
            match i { 
                0 => {
                    addition = c == 'R';
                },
                1 => {
                    number =  c.to_digit(10).unwrap() as i32 ;
                },
                2 => {
                    number =  c.to_digit(10).unwrap() as i32 + number*10; 
                },
                3 => {
                    // Add the full rotations to count
                    count += number / 10;
                    // we only care about the last two digits 
                    // if number is 112
                    // we only need to get the x12 as 12
                    number =  c.to_digit(10).unwrap() as i32 + ((number%10)*10); 
                },
                _ => {
                    panic!("Incorrect input")

                }
            }
        }
        starting_dial = dial;
        dial += if addition {1} else { -1 } * number;
        if dial < 0 {
            dial += 100;
            // Correct for if we have started at 0
            // to avoid double counting it 
            if starting_dial != 0 {
                count += 1;
            }
        }else if dial > 99 {
            dial -= 100;
            count += 1;
        }else {
            count += if dial == 0 {1} else { 0 };
        }
    }

    print!("The password is {:?}",count);



    Ok(())
    


}
