use std::collections::HashSet;
use std::error::Error;
use std::fs::read_to_string;

#[derive(Clone, Copy)]
struct Point(i32, i32);

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt").unwrap();

    let mut head = Point(0, 0);
    let mut tail = vec![Point(0, 0); 9];
    let mut positions = HashSet::new();
    positions.insert((0, 0));

    for line in input.lines() {
        let (first, second) = line.split_once(" ").unwrap();
        let second = second.parse::<u32>().unwrap();
        let direction = match first {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => panic!("Wrong op!"),
        };

        for _ in 0..second {
            head.0 += direction.0;
            head.1 += direction.1;
            tail[0] = mov_tail(head, tail[0]);
            for i in 0..8 {
                tail[i + 1] = mov_tail(tail[i], tail[i + 1]);
            }
            positions.insert((tail[8].0, tail[8].1));
        }
    }

    println!("{}", positions.len());

    Ok(())
}

fn mov_tail(head: Point, mut tail: Point) -> Point {
    match (head.0 - tail.0, head.1 - tail.1) {
        (-1..=1, -1..=1) => (),
        (2, 0) => tail.0 += 1,
        (2, 1) => (tail.0, tail.1) = (tail.0 + 1, tail.1 + 1),
        (2, 2) => (tail.0, tail.1) = (tail.0 + 1, tail.1 + 1),
        (1, 2) => (tail.0, tail.1) = (tail.0 + 1, tail.1 + 1),
        (0, 2) => tail.1 += 1,
        (-1, 2) => (tail.0, tail.1) = (tail.0 - 1, tail.1 + 1),
        (-2, 2) => (tail.0, tail.1) = (tail.0 - 1, tail.1 + 1),
        (-2, 1) => (tail.0, tail.1) = (tail.0 - 1, tail.1 + 1),
        (-2, 0) => tail.0 -= 1,
        (-2, -1) => (tail.0, tail.1) = (tail.0 - 1, tail.1 - 1),
        (-2, -2) => (tail.0, tail.1) = (tail.0 - 1, tail.1 - 1),
        (-1, -2) => (tail.0, tail.1) = (tail.0 - 1, tail.1 - 1),
        (0, -2) => tail.1 -= 1,
        (1, -2) => (tail.0, tail.1) = (tail.0 + 1, tail.1 - 1),
        (2, -2) => (tail.0, tail.1) = (tail.0 + 1, tail.1 - 1),
        (2, -1) => (tail.0, tail.1) = (tail.0 + 1, tail.1 - 1),
        _ => panic!("Invalid difference"),
    };
    return tail;
}
