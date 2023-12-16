use std::fs;

const FILE_PATH: &str = "input1.txt";
const DIGITS: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn main() {

    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");

    let mut sum: u32 = 0;

    for line in lines {

        let mut first_digit: u32 = 0;
        let mut last_digit: u32 = 0;

        let mut first_digit_index: i32 = -1;
        let mut last_digit_index: i32 = -1;

        for (i, digit) in DIGITS.iter().enumerate() {
            if let Some(index) = line.find(digit) {
                if index < first_digit_index as usize || first_digit_index == -1 {
                    first_digit_index = index  as i32;
                    first_digit = i as u32;
                }
            }
            if let Some(index) = line.rfind(digit) {
                if index > last_digit_index as usize || last_digit_index == -1 {
                    last_digit_index = index  as i32;
                    last_digit = i as u32;
                }
            }
        }

        for (i, x) in line.chars().enumerate() {    
            if let Some(digit_value) = x.to_digit(10) {
                if i < first_digit_index as usize || first_digit_index == -1 {
                    first_digit = digit_value;
                    first_digit_index = i as i32;
                }
                if i > last_digit_index as usize || last_digit_index == -1 {
                    last_digit = digit_value;
                    last_digit_index = i as i32;
                }
            }
        }

        sum += first_digit*10 + last_digit;

    }
    println!("{}", sum);
}