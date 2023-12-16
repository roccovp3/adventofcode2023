use std::fs;

const FILE_PATH: &str = "input1.txt";

fn main() {

    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");

    let mut sum: u32 = 0;

    for line in lines {

        let mut first_digit: u32 = 0;
        let mut last_digit: u32 = 0;

        for x in line.chars() {    
            if let Some(digit_value) = x.to_digit(10) {
                first_digit = digit_value;
                break;
            }
        }
        for x in line.chars().rev() {
            if let Some(digit_value) = x.to_digit(10) {
                last_digit = digit_value;
                break;
            }
        }

        sum += first_digit*10 + last_digit;

    }
    println!("{}", sum);
}