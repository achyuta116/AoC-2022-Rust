use std::collections::HashSet;
use std::error::Error;
use std::fs::read_to_string;

#[derive(Hash, Eq, PartialEq)]
struct Sensor {
    x: i32,
    y: i32,
    range: i32,
}

#[derive(Hash, Eq, PartialEq)]
struct Beacon {
    x: i32,
    y: i32,
}

fn extract_integers(line: &str) -> Vec<i32> {
    let mut it = line.chars();
    let mut ret = vec![];
    while let Some(ch) = it.next() {
        match ch {
            '0'..='9' => {
                let mut number = String::from(ch);
                while let Some(ch) = it.next() {
                    match ch {
                        '0'..='9' => {
                            number.push(ch);
                        }
                        _ => break,
                    }
                }
                ret.push(number.parse::<i32>().unwrap());
            }
            _ => (),
        }
    }
    ret
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt").unwrap();
    let mut input = input.lines();

    let mut sensors: HashSet<Sensor> = HashSet::new();
    let mut beacons: HashSet<Beacon> = HashSet::new();
    while let Some(line) = input.next() {
        let numbers = extract_integers(line);
        sensors.insert(Sensor {
            x: numbers[0],
            y: numbers[1],
            range: i32::abs(numbers[0] - numbers[2]) + i32::abs(numbers[1] - numbers[3]),
        });
        beacons.insert(Beacon {
            x: numbers[2],
            y: numbers[3],
        });
    }

    let row = 2_000_000;
    let mut marked: HashSet<i32> = HashSet::new();
    for sensor in sensors.iter() {
        if i32::abs(row - sensor.y) >= 0 {
            if sensor.range < i32::abs(row - sensor.y) {
                continue;
            }
            let leftover = sensor.range - i32::abs(row - sensor.y);
            for x in -leftover..=leftover {
                marked.insert(sensor.x + x);
            }
        }
    }

    println!(
        "{}",
        marked.len() - beacons.iter().filter(|&b| b.y == row).count()
    );

    Ok(())
}
