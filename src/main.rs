use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let path = Path::new("./input.txt");
    let display = path.display();
    println!("{}", display);
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(w) => panic!("{}", w),
    };

    let mut elves = vec![];

    let mut current = 0;
    for line in io::BufReader::new(file).lines() {
        if let Ok(ip) = line {
            if let Ok(calorie_count) = ip.parse::<i32>() {
                current += calorie_count;
            } else {
                elves.push(current);
                current = 0;
            }
        }
    }

    elves.sort();
    elves.reverse();
    let top_three_sum: i32 = elves[0..3].iter().sum();

    println!("{}", top_three_sum);
}
