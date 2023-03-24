use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
    path::Path, cmp::min,
};

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("./input.txt");
    let file = File::open(&path)?;

    let mut lines = io::BufReader::new(&file).lines();

    let mut total_occupied = 0;
    while let Some(Ok(line)) = lines.next() {
        let (first, _) = line.split_once(" ").unwrap();
        match first {
            "$" => continue,
            "dir" => continue,
            _ => ()
        }
        let first = first.parse::<u32>().unwrap();
        total_occupied += first;
    }

    let path = Path::new("./input.txt");
    let file = File::open(&path)?;

    let mut lines = io::BufReader::new(&file).lines();

    lines.next().unwrap().unwrap();

    let mut st = vec![0];
    let mut min_dir = 40_000_000;

    while let Some(Ok(line)) = lines.next() {
        let instr: Vec<_> = line
            .split(" ")
            .into_iter()
            .map(|el| el.to_owned())
            .collect();
        match instr[0].as_ref() {
            "$" => {
                match instr[1].as_ref() {
                    "ls" => (),
                    "cd" => {
                        if instr[2] == ".." {
                            if let Some(popped) = st.pop(){
                                if popped > total_occupied - 40_000_000 {
                                    min_dir = min(popped, min_dir);
                                }
                                *st.last_mut().unwrap() += popped;
                            }
                        } else {
                            st.push(0);
                        }
                    },
                    _ => ()
                }
            }
            "dir" => (),
            _ => {
                let last = st.last_mut().unwrap();
                *last = *last + instr[0].parse::<u32>().unwrap();
            }
        }
    }

    st.reverse();
    let mut current = 0;
    for el in st.into_iter() {
        current += el;
        if el > total_occupied - 40_000_000 {
            min_dir = min(el, min_dir);
        } else if current > total_occupied - 40_000_000{
            min_dir = min(current, min_dir);
        }
    }

    println!("{}", min_dir);

    Ok(())
}
