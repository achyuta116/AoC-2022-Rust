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

    let mut score = 0;

    for line in io::BufReader::new(file).lines() {
        if let Ok(ip) = line {
            let choices = ip.split_once(" ");
            if let Some((a, b)) = choices {
                score += calculate_score(a, b);
            }
        }
    }

    println!("{}", score);
}

fn calculate_score(a: &str, b: &str) -> i32 {
    match (a, b) {
        ("A", "X") => 0 + 3,
        ("A", "Y") => 3 + 1,
        ("A", "Z") => 6 + 2,
        ("B", "X") => 0 + 1,
        ("B", "Y") => 3 + 2,
        ("B", "Z") => 6 + 3,
        ("C", "X") => 0 + 2,
        ("C", "Y") => 3 + 3,
        ("C", "Z") => 6 + 1,
        _ => panic!("Incorrect Combination"),
    }
}
