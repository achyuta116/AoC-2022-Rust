use std::collections::HashSet;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt").unwrap();

    let mut trees = vec![];

    for line in input.lines() {
        let treeline = line.chars().collect::<Vec<_>>();
        trees.push(treeline);
    }

    let mut visible: HashSet<(u32, u32)> = HashSet::new();

    // l to r
    for i in 0..trees.len() {
        visible.insert((i as u32, 0));
        let mut mx = trees[i][0].to_digit(10).unwrap();
        for j in 1..trees[0].len() {
            if trees[i][j].to_digit(10).unwrap() > mx {
                mx = trees[i][j].to_digit(10).unwrap();
                visible.insert((i as u32, j as u32));
            }
        }
    }
    // r to l
    for i in 0..trees.len() {
        visible.insert((i as u32, (trees.len() - 1) as u32));
        let mut mx = trees[i][trees.len() - 1].to_digit(10).unwrap();
        for j in (0..trees[0].len() - 1).rev() {
            if trees[i][j].to_digit(10).unwrap() > mx {
                mx = trees[i][j].to_digit(10).unwrap();
                visible.insert((i as u32, j as u32));
            }
        }
    }
    // u to d
    for j in 0..trees[0].len() {
        visible.insert((0, j as u32));
        let mut mx = trees[0][j].to_digit(10).unwrap();
        for i in 1..trees.len() {
            if trees[i][j].to_digit(10).unwrap() > mx {
                mx = trees[i][j].to_digit(10).unwrap();
                visible.insert((i as u32, j as u32));
            }
        }
    }
    // d to u
    for j in 0..trees.len() {
        visible.insert(((trees.len() - 1) as u32, j as u32));
        let mut mx = trees[trees.len() - 1][j].to_digit(10).unwrap();
        for i in (0..trees[0].len()-1).rev() {
            if trees[i][j].to_digit(10).unwrap() > mx {
                mx = trees[i][j].to_digit(10).unwrap();
                visible.insert((i as u32, j as u32));
            }
        }
    }

    println!("{}", visible.len());

    Ok(())
}
