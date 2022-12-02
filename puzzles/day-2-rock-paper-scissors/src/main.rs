use std::{io::{self, BufRead}, fs::File, path::Path};

fn main() {
    println!("Score: {}", calculate_score(read_lines("input").unwrap()));
}

fn calculate_score(lines: io::Lines<io::BufReader<File>>) -> u32 {
    let mut score = 0;

    for line in lines {
        if let Ok(l) = line {
            let mut it = l.split_whitespace();
            
            let round = match (it.next().unwrap(), it.next().unwrap()) {
                ("A", "X") => 3,
                ("A", "Y") => 4,
                ("A", "Z") => 8,
                ("B", "X") => 1,
                ("B", "Y") => 5,
                ("B", "Z") => 9,
                ("C", "X") => 2,
                ("C", "Y") => 6,
                ("C", "Z") => 7,
                _ => 0
            };
            score += round;
        }
    }

    score
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}