use std::{path::Path, fs::File, io::{self, BufRead}};
use std::cmp::max;

fn main() {
    let path = Path::new("./input.txt");
    let display = path.display();
    println!("{}", display);
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(w) => panic!("{}", w),
    };

    let mut current = 0;
    let mut mx = 0;
    for line in io::BufReader::new(file).lines() {
        if let Ok(ip) = line {
            println!("{}", ip);
            if let Ok(calorie_count) = ip.parse::<i32>() {
                current += calorie_count;
            } else {
                mx = max(mx, current);
                current = 0;
            }
        }
    }

    println!("{}", mx);
}
