use std::fs;

const FILE_PATH: &str = "input2.txt";
const COLORS: [(&str, u32); 3] = [("red", 12), ("green", 13), ("blue", 14)];

fn main() {

    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let lines = contents.lines();
    
    let mut sum = 0;

    for (i, line) in lines.enumerate() {
        let game_id: u32 = i as u32 + 1;
        let results: Vec<&str> = line
            .split(':')
            .nth(1)
            .unwrap_or_default()
            .split(';')
            .collect();
        
        let mut possible: bool = true;
        
        for result in results {
            let color_counts: Vec<&str> = result
                .split(',')
                .collect();
            for count in color_counts {
                for color in COLORS {
                    if let Some(color_index) = count.find(color.0) {
                        if count[1..color_index-1].parse::<u32>().unwrap() > color.1 {
                            possible = false;
                        }
                    }
                }
            }
        }
    
        if possible {
            sum += game_id;
        }
    }
    println!("{}", sum);
}