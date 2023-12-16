use std::fs;

const FILE_PATH: &str = "input3.txt";
const NOT_SYMBOLS: [char; 11] = ['.', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn main() {

    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split('\n').collect();

    let mut i = 0;
    let mut j = 0;

    while i < lines.len() - 1 {
        j = 0;
        while j < lines[0].len() {
            if !NOT_SYMBOLS.contains(&lines[i].chars().nth(j).unwrap()) {
                println!("symbol at {}, {}", i, j);
            }
            j += 1;
        }
        i += 1;
    }
}