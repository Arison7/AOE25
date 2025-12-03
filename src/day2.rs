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
