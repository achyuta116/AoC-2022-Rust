use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("./input.txt");
    let file = File::open(&path)?;

    let mut lines = io::BufReader::new(&file).lines();

    lines.next().unwrap().unwrap();

    let mut st = vec![0];
    let mut total = 0;

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
                            let fin = *st.last().unwrap();
                            if fin < 100000 {
                                total += fin;
                            }
                            st.pop();
                            if let Some(last) = st.last_mut() {
                                *last += fin;
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
                *last = *last + instr[0].parse::<i32>().unwrap();
            }
        }
    }

    println!("{}", total);

    Ok(())
}
