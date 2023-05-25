use std::cell::Cell;
use std::collections::VecDeque;
use std::error::Error;
use std::fs::read_to_string;

struct Item {
    ch: char,
    visited: Cell<bool>,
}

fn bfs(end: (i32, i32), chars: &Vec<Vec<Item>>) -> usize {
    let mut q = VecDeque::new();
    let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    chars[end.0 as usize][end.1 as usize].visited.set(true);
    q.push_back((end.0 as usize, end.1 as usize, 0));
    while !q.is_empty() {
        let (x, y, dist) = q.pop_front().unwrap();
        let source = chars[x][y].ch;

        for dir in dirs.iter() {
            let xx = x as i32 + dir.0;
            let yy = y as i32 + dir.1;
            if xx >= 0 && xx < chars.len() as i32 && yy >= 0 && yy < chars[0].len() as i32 {
                let xx = xx as usize;
                let yy = yy as usize;

                let child = chars[xx][yy].ch;
                if chars[xx][yy].visited.get() {
                    continue;
                }

                if child == 'S' {
                    return dist + 1;
                }

                if (source as i32 - child as i32 <= 1 && source != 'E')
                    || (source == 'E' && child == 'z')
                {
                    if child == 'a' {
                        return dist + 1;
                    }
                    chars[xx][yy].visited.set(true);
                    q.push_back((xx as usize, yy as usize, dist + 1));
                }
            }
        }
    }

    return 0;
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt").unwrap();
    let mut input = input.lines();
    let mut chars: Vec<Vec<Item>> = vec![];

    let mut start: (i32, i32) = (0, 0);
    let mut lindex = 0 as usize;
    while let Some(line) = input.next() {
        if let Some(index) = line.find("E") {
            start = (lindex as i32, index as i32);
        }

        chars.push(
            line.chars()
                .map(|ch| Item {
                    ch,
                    visited: false.into(),
                })
                .collect(),
        );
        lindex += 1;
    }

    println!("{}", bfs(start, &chars));
    Ok(())
}
