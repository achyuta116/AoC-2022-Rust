use std::{
    fs::File,
    io::{self, BufRead},
    path::Path, error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("./input.txt");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(w) => panic!("{}", w),
    };

    let mut lines = io::BufReader::new(file).lines();

    let mut count = 0;

    while let Some(Ok(line)) = lines.next() {
        let (first_pair, second_pair) = line.split_once(",").unwrap();
        let (first_start, first_end) = first_pair.split_once("-").unwrap();
        let (second_start, second_end) = second_pair.split_once("-").unwrap();
        if first_start.parse::<u32>()? <= second_start.parse::<u32>()?
            && first_end.parse::<u32>()? >= second_end.parse::<u32>()?
        {
            count += 1;
        } else if first_start.parse::<u32>()? >= second_start.parse::<u32>()?
            && first_end.parse::<u32>()? <= second_end.parse::<u32>()?
        {
            count += 1;
        }
    }

    println!("{}", count);
    Ok(())
}
