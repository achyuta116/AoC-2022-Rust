use std::{
    fs::File,
    io::{self, BufRead},
    path::Path, collections::HashSet,
};

fn main() {
    let path = Path::new("./input.txt");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(w) => panic!("{}", w),
    };


    let mut sum_priorities = 0;

    for line in io::BufReader::new(file).lines() {
        let mut first_set: HashSet<char> = HashSet::new();
        let mut second_set: HashSet<char> = HashSet::new();
        let line = line.unwrap();
        for (index, char) in line.chars().enumerate() {
            if index < line.len()/2 {
                second_set.insert(char);
                continue
            }
            first_set.insert(char);
        }
        let intersection = first_set.intersection(&second_set);
        for char in intersection.into_iter() {
            sum_priorities += get_value(char);
        }
    }

    println!("{}", sum_priorities);
}

fn get_value(char: &char) -> u32 {
    match char {
        'A'..='Z' => char.to_owned() as u32 - 'A' as u32 + 27,
        'a'..='z' => char.to_owned() as u32 - 'a' as u32 + 1,
        _ => panic!("Unexpected Character")
    }
}
