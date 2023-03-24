use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("./input.txt");
    let file = File::open(&path)?;

    let mut message = String::new();
    io::BufReader::new(file).read_line(&mut message).unwrap();
    let message = message.chars().collect::<Vec<char>>();

    let mut m: HashMap<char, u32> = HashMap::new();

    let mut found_index = 0;
    for i in 0..14 {
        if let Some(ch) = m.get(&message[i]) {
            m.insert(message[i], ch + 1);
        } else {
            m.insert(message[i], 1);
        }
    }

    for i in 14..message.len() {
        if m.keys().len() == 14 {
            found_index = i;
            break;
        }

        let prev = m.get(&message[i - 14]).unwrap();
        if *prev == 1 {
            m.remove(&message[i - 14]);
        } else {
            m.insert(message[i - 14], prev - 1);
        }

        if let Some(num) = m.get(&message[i]) {
            m.insert(message[i], num + 1);
        } else {
            m.insert(message[i], 1);
        }
    }

    println!("{}", found_index);

    Ok(())
}
