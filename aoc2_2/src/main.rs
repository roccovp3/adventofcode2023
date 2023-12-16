use std::fs;

const FILE_PATH: &str = "input2.txt";

fn main() {

    let mut colors: [(&str, u32); 3] = [("red", 0), ("green", 0), ("blue", 0)];
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let lines = contents.lines();
    
    let mut sum = 0;

    for (i, line) in lines.enumerate() {
        let _game_id: u32 = i as u32 + 1;

        colors[0].1 = 0;
        colors[1].1 = 0;
        colors[2].1 = 0;

        let results: Vec<&str> = line
            .split(':')
            .nth(1)
            .unwrap_or_default()
            .split(';')
            .collect();
                
        for result in results {
            let color_counts: Vec<&str> = result
                .split(',')
                .collect();
            for count in color_counts {
                for color in &mut colors {
                    if let Some(color_index) = count.find(color.0) {
                        let num = count[1..color_index-1].parse::<u32>().unwrap();
                        if num > color.1 {
                            color.1 = num;
                        }
                    }
                }
            }
        }
        sum += (colors[0].1)*(colors[1].1)*(colors[2].1);
    }
    println!("{}", sum);
}