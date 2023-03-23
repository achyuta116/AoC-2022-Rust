use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("./input.txt");
    let file = File::open(&path)?;

    let mut lines = io::BufReader::new(file).lines();

    let mut stacks: Vec<Vec<char>> = vec![vec![]; 9];

    for _ in 0..8 {
        let line = lines
            .next()
            .unwrap()
            .unwrap()
            .chars()
            .collect::<Vec<char>>();
        for c in 0..9 {
            match line[c * 4 + 1] {
                'A'..='Z' => stacks[c].push(line[c * 4 + 1]),
                ' ' => (),
                _ => panic!("Bad Character"),
            }
        }
    }
    lines.next();
    lines.next();

    stacks.iter_mut().for_each(|stack| stack.reverse());
    while let Some(Ok(line)) = lines.next() {
        let instruction: Vec<_> = line.split(" ").collect();
        let number = instruction[1].parse::<u32>()?;
        let from = instruction[3].parse::<usize>()? - 1;
        let to = instruction[5].parse::<usize>()? - 1;

        let mut temp = vec![];
        for _ in 0..number {
            let block = stacks[from].pop().unwrap();
            temp.push(block);
        }

        temp.reverse();
        for block in temp.into_iter() {
            stacks[to].push(block);
        }
    }

    for stack in stacks.iter() {
        print!("{}", stack.last().unwrap());
    }

    Ok(())
}
