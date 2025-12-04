use std::fs::{read_to_string, File};

fn get_file() -> Result<String, std::io::Error> {
    let line = read_to_string("input_files/day2.txt")?;
    Ok(line)
}

pub fn part1() {
    let mut invalid_id = 0;
    let mut binding = get_file().expect("Failed to load file");
    let mut content = binding.as_mut_str().trim();
    let mut size: u32;
    let base: u32 = 10;
    print!("{:?}", content);
    while let Some(i) = content.find(',') {
        // Get both numberes
        let start: i64 = content[..content.find('-').expect("failed to find -")]
            .parse()
            .expect("failed to parse to i64");
        print!("{:?}", start);
        let end: i64 = content[content.find('-').expect("failed to find -") + 1..i]
            .parse()
            .expect("failed to parse to i32");
        print!("-{:?}\n", end);
        // Iterate over the range
        for i in start..=end {
            // Get the amount of digits in the number
            size = (i.checked_ilog10().unwrap_or(0) + 1) / 2;
            invalid_id += if (i / base.pow(size) as i64) - (i % base.pow(size) as i64) == 0 {
                i
            } else {
                0
            };
        }
        // Get the next slice
        content = &content[i + 1..];
    }
    print!("\ninvalid_ids: {:?}", invalid_id)
}

pub fn part2() {
    let mut invalid_id = 0;
    let mut binding = get_file().expect("failed to load file");
    let mut content = binding.as_mut_str().trim();
    let mut size: u64;
    let base: u64 = 10;
    let mut position: Vec<u64> = Vec::with_capacity(10);
    let mut factors: Vec<u64>;

    while let Some(i) = content.find(',') {
        // get both numberes
        let start: u64 = content[..content.find('-').expect("failed to find -")]
            .parse()
            .expect("failed to parse to u64");
        //print!("{:?}", start);
        let end: u64 = content[content.find('-').expect("failed to find -") + 1..i]
            .parse()
            .expect("failed to parse to u64");
        //print!("-{:?}\n", end);
        // iterate over the range
        for i in start..=end {
            //print!("number: {:?}\n", i);
            // get the amount of digits in the number
            size = i.checked_ilog10().unwrap_or(0) as u64 + 1;

            // get all the factors of the size
            // if the main focus here would be omtimalisation i would save the outputs
            // into a dictionary so we don't need to call the function every time
            // however since size doesn't exceed like 10 i think it's fine to leave like this
            factors = list_factors(size);
            /*
            print!(
                "
                \nfactors: {:?}\n",
                factors
            );
            */
            for factor in factors {
                if factor == size {
                    continue;
                }
                //print!("factor: {:?}\n",factor);
                position.clear();
                let mut temp = i;
                for j in (factor..=size).step_by((factor) as usize) {
                    // remove the slice of number from the number and add it to the vec
                    let slice_of_number = temp % base.pow(j as u32) / base.pow((j - factor) as u32);
                    temp -= slice_of_number;
                    position.push(slice_of_number);
                    //get the part of the number
                }
                // check if all the values are the same
                if position.windows(2).all(|w| w[0] == w[1]) {
                    invalid_id += i;
                    break;
                }
                /*
                print!(
                    "
                    \nposition: {:?}\n",
                    position
                );
                */
            }
        }
        // get the next slice
        content = &content[i + 1..];
    }
    print!("\ninvalid_ids: {:?}", invalid_id)
}

fn list_factors(number: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    let mut i: u64 = 1;
    while i * i <= number {
        if number % i == 0 {
            factors.push(i);
            if i * i != number {
                factors.push(number / i);
            }
        }
        i += 1;
    }
    factors
}
