use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file_path = "./src/input.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            let mut first_digit: Option<char> = None;
            let mut last_digit: Option<char> = None;

            for c in line.chars() {
                if c.is_digit(10) {
                    first_digit = Some(c);
                    break;
                }
            }

            for c in line.chars().rev() {
                if c.is_digit(10) {
                    last_digit = Some(c);
                    break;
                }
            }

            let concatenated_digits = match (first_digit, last_digit) {
                (Some(first), Some(last)) => format!("{}{}", first, last),
                _ => "".to_string(),
            };

            if let Ok(digit_sum) = concatenated_digits.parse::<i32>() {
                sum += digit_sum;
            }
        }
    }

    println!("{}", sum);

    Ok(())
}
