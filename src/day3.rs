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

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct Item {
    digit: i64,
    index: usize,
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        // highest digit first
        match self.digit.cmp(&other.digit) {
            Ordering::Equal => {
                // lower index first
                other.index.cmp(&self.index)
            }
            other => other,
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
pub fn part2() {
    let mut output: i64 = 0;
    let mut heap = BinaryHeap::new();
    let mut max;
    let mut max_index = 0;
    let mut count;
    let window = 12;
    for line in read_to_string("input_files/day3.txt").unwrap().lines() {
        println!("{:?}", line);
        max = 0;
        let mut chars = line.chars();
        // Find the higesht leading number and queue all the numbers behind ig
        for i in 0..=(line.chars().count() - window) {
            let c = chars.next().unwrap();
            let number = c.to_digit(10).expect("failed to convert to digit") as i64;
            if max < number {
                max = number;
                max_index = i;
                heap.clear();
            } else {
                heap.push(Item {
                    digit: number,
                    index: i,
                });
            }
        }
        count = max;
        let mut i = (line.len() - window) + 1;
        for c in chars {
            let mut number = c.to_digit(10).expect("failed to convert to digit") as i64;
            let popped_index = max_index;
            while popped_index >= max_index {
                if let Some(top) = heap.pop() {
                    max = top.digit;
                    max_index = top.index;
                } else {
                    max = 0;
                    break;
                }
            }
            // We can just take the rest of the number
            if max < number {
                for c in line[i..line.len()].chars() {
                    number = c.to_digit(10).expect("failed to convert to digit") as i64;
                    count *= 10;
                    count += number;
                }
                break;
            } else {
                // Add the highest found number
                count *= 10;
                count += max;
                heap.push(Item {
                    digit: number,
                    index: i,
                })
            }
            i += 1;
        }
        output += count;
        //println!("{:?}", count);
    }
    print!("{:?}", output);
}
