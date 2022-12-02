
use std::{fs::File, io::{self, BufRead}, path::Path, collections::BinaryHeap};

fn main() {
    if let Ok(lines) = read_lines("input") {
        let max_cal = find_max_calories(lines);
        print!("{}", max_cal);
    }
}

fn find_max_calories(lines: io::Lines<io::BufReader<File>>) -> u32 {
    let mut current = 0;
    
    let mut heap:BinaryHeap<u32> = BinaryHeap::new();

    for line in lines {
        if let Ok(l) = line {
            if l.is_empty() {
                heap.push(current);
                current = 0;
            } else {
                current += l.parse::<u32>().unwrap_or_default();
            }
        }
    }
    heap.pop().unwrap_or_default()
    + heap.pop().unwrap_or_default()
    + heap.pop().unwrap_or_default()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}