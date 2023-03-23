use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let path = Path::new("./input.txt");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(w) => panic!("{}", w),
    };

    let mut lines = io::BufReader::new(file).lines();
    let mut sum_priorities = 0;

    while let Some(Ok(line)) = lines.next() {
        let mut first_set: HashSet<char> = HashSet::new();
        for char in line.chars() {
            first_set.insert(char);
        }

        let mut second_set: HashSet<char> = HashSet::new();
        for char in lines.next().unwrap().unwrap().chars() {
            second_set.insert(char);
        }

        let mut third_set: HashSet<char> = HashSet::new();
        for char in lines.next().unwrap().unwrap().chars() {
            third_set.insert(char);
        }

        let intersection = first_set
            .intersection(&second_set)
            .cloned()
            .collect::<HashSet<char>>();
        let intersection = intersection.intersection(&third_set);

        for char in intersection.into_iter() {
            sum_priorities += get_value(&char);
        }
    }

    println!("{}", sum_priorities);
}

fn get_value(char: &char) -> u32 {
    match char {
        'A'..='Z' => char.to_owned() as u32 - 'A' as u32 + 27,
        'a'..='z' => char.to_owned() as u32 - 'a' as u32 + 1,
        _ => panic!("Unexpected Character"),
    }
}
