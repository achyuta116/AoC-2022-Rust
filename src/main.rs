use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

enum Particle {
    Sand,
    Rock,
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt").unwrap();
    let mut input = input.lines();

    let mut scan: HashMap<(i32, i32), Particle> = HashMap::new();
    let mut low_x = std::i32::MAX;
    let mut high_x = std::i32::MIN;

    while let Some(line) = input.next() {
        let mut rock_points = line.split(" -> ");
        let mut first = rock_points.next().unwrap();

        while let Some(second) = rock_points.next() {
            let (second_x, second_y) = second.split_once(",").unwrap();
            let (first_x, first_y) = first.split_once(",").unwrap();

            let second_x = second_x.parse::<i32>().unwrap();

            let second_y = second_y.parse::<i32>().unwrap();
            high_x = std::cmp::max(high_x, second_y);
            low_x = std::cmp::min(low_x, second_y);

            let first_x = first_x.parse::<i32>().unwrap();

            let first_y = first_y.parse::<i32>().unwrap();
            high_x = std::cmp::max(high_x, first_y);
            low_x = std::cmp::min(low_x, first_y);

            if first_x == second_x {
                let it = if first_y < second_y {
                    first_y..=second_y
                } else {
                    second_y..=first_y
                };
                for y in it {
                    scan.insert((first_x, y), Particle::Rock);
                }
            } else if first_y == second_y {
                let it = if first_x < second_x {
                    first_x..=second_x
                } else {
                    second_x..=first_x
                };
                for x in it {
                    scan.insert((x, first_y), Particle::Rock);
                }
            }
            first = second;
        }
    }

    loop {
        struct Pos {
            x: i32,
            y: i32,
        }
        let mut sand_pos = Pos { x: 500, y: 0 };

        let mut flag = true;
        while sand_pos.y < high_x + 2 {
            if sand_pos.y == high_x + 1 && scan.get(&(sand_pos.x, sand_pos.y)).is_none() {
                flag = false;
                scan.insert((sand_pos.x, sand_pos.y), Particle::Sand);
                break
            }

            if scan.get(&(sand_pos.x, sand_pos.y + 1)).is_none() {
                sand_pos.y += 1;
                continue;
            }

            if scan.get(&(sand_pos.x - 1, sand_pos.y + 1)).is_none() {
                sand_pos.x -= 1;
                sand_pos.y += 1;
                continue
            }

            if scan.get(&(sand_pos.x + 1, sand_pos.y + 1)).is_none() {
                sand_pos.x += 1;
                sand_pos.y += 1;
                continue;
            }

            if scan.get(&(sand_pos.x, sand_pos.y)).is_none() {
                scan.insert((sand_pos.x, sand_pos.y), Particle::Sand);
                flag = false;
                break;
            }

            if sand_pos.x == 500 && sand_pos.y == 0 {
                break
            }
        }

        if flag {
            break
        }
    }

    println!("{}", scan.iter().filter(|(_, &ref v)| match v {
        Particle::Sand => true,
        _ => false
    }).count());

    Ok(())
}
