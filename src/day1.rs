use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let filename = "./day1.txt";
    let path = Path::new(filename);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let reader = io::BufReader::new(file);
    let mut sum = 0;
    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        println!("Line {}: {}", index + 1, line);
        let number = extract_digits(&line);
        sum += number
    }
    println!("Sum: {}", sum);

    Ok(())
}

fn extract_digits(s: &str) -> u32 {
    let digits: Vec<u32> = s.chars().filter_map(|c| c.to_digit(10)).collect();
    match digits.len() {
        0 => 0,
        1 => digits[0] * 10 + digits[0],
        _ => digits[0] * 10 + *digits.last().unwrap(),
    }
}
