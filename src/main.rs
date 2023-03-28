use std::collections::HashSet;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt").unwrap();


    let  (mut head_x, mut head_y) = (0, 0);
    let  (mut tail_x, mut tail_y) = (0, 0);
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
            _ => panic!("Wrong op!")
        };

        for _ in 0..second {
            head_x += direction.0;
            head_y += direction.1;
            match (head_x - tail_x, head_y - tail_y) {
                (-1..=1, -1..=1) => (),
                (2, 0) => tail_x += 1,
                (2, 1) => (tail_x, tail_y) = (tail_x + 1,tail_y + 1),
                (2, 2) => (tail_x, tail_y) = (tail_x + 1,tail_y + 1),
                (1, 2) => (tail_x, tail_y) = (tail_x + 1,tail_y + 1),
                (0, 2) => tail_y += 1,
                (-1, 2) => (tail_x, tail_y) = (tail_x - 1,tail_y + 1),
                (-2, 2) => (tail_x, tail_y) = (tail_x - 1,tail_y + 1),
                (-2, 1) => (tail_x, tail_y) = (tail_x - 1,tail_y + 1),
                (-2, 0) => tail_x -= 1,
                (-2, -1) => (tail_x, tail_y) = (tail_x - 1,tail_y - 1),
                (-2, -2) => (tail_x, tail_y) = (tail_x - 1,tail_y - 1),
                (-1, -2) => (tail_x, tail_y) = (tail_x - 1,tail_y - 1),
                (0, -2) => tail_y -= 1,
                (1, -2) => (tail_x, tail_y) = (tail_x + 1, tail_y - 1),
                (2, -2) => (tail_x, tail_y) = (tail_x + 1, tail_y - 1),
                (2, -1) => (tail_x, tail_y) = (tail_x + 1, tail_y - 1),
                _ => panic!("Invalid difference")
            }
            positions.insert((tail_x, tail_y));
        }
    }
    

    println!("{}", positions.len());


    Ok(())
}
