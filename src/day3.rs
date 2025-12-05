use std::cmp;
use std::fs::{read_to_string, File};

pub fn part1() {
    let mut count = 0;
    let mut max_before_max: i32;
    let mut max: i32;
    let mut max_index: i32;
    let mut max_after_max: i32;

    for line in read_to_string("input_files/day3.txt").unwrap().lines() {
        // Reset all
        max = 0;
        max_before_max = 0;
        max_index = 0;
        max_after_max = -100;
        for (i, c) in line.chars().enumerate() {
            let i = i as i32;
            let number = c.to_digit(10).expect("failed to convert to digit") as i32;
            if (number > max) {
                max_before_max = max;
                max = number;
                max_index = i;
                max_after_max = -100;
            } else if (i < max_index && number > max_before_max) {
                max_before_max = number;
            } else if (i > max_index && number > max_after_max) {
                max_after_max = number
            }
        }
        count += cmp::max(max * 10 + max_after_max, max_before_max * 10 + max);
    }
    println!("{:?}", count)
}
