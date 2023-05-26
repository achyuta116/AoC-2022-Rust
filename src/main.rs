use std::collections::HashSet;
use std::error::Error;
use std::fs::read_to_string;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Sensor {
    x: i32,
    y: i32,
    range: i32,
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Beacon {
    x: i32,
    y: i32,
}

fn extract_integers(line: &str) -> Vec<i32> {
    let mut it = line.chars();
    let mut ret = vec![];
    while let Some(ch) = it.next() {
        match ch {
            '0'..='9' | '-' => {
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

fn merge(ranges: &mut Vec<(i32, i32)>) {
    ranges.sort();
    let mut ans: Vec<(i32, i32)> = vec![];
    ans.push(ranges[0]);
    for el in ranges.iter().skip(1) {
        if el.0 <= ans.last().unwrap().1 {
            ans.last_mut().unwrap().1 = i32::max(ans.last().unwrap().1, el.1);
        } else if el.0 == ans.last().unwrap().1 + 1 {
            ans.last_mut().unwrap().1 = el.1;
        } else {
            ans.push(*el);
        }
    }
    *ranges = ans;
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt").unwrap();
    let mut input = input.lines();

    let mut sensors: HashSet<Sensor> = HashSet::new();
    let mut beacons: HashSet<Beacon> = HashSet::new();
    while let Some(line) = input.next() {
        let numbers = extract_integers(line);
        let sensor = Sensor {
            x: numbers[0],
            y: numbers[1],
            range: i32::abs(numbers[0] - numbers[2]) + i32::abs(numbers[1] - numbers[3]),
        };
        sensors.insert(sensor);

        let beacon = Beacon {
            x: numbers[2],
            y: numbers[3],
        };
        beacons.insert(beacon);
    }

    let rows = 4_000_000;
    for row in 0..=rows {
        let mut ranges = vec![];
        for sensor in sensors.iter() {
            let leftover = sensor.range - i32::abs(sensor.y - row);
            if leftover < 0 {
                continue;
            }
            let left = i32::max(0, sensor.x - leftover);
            let right = i32::min(rows, sensor.x + leftover);
            ranges.push((left, right));
        }
        merge(&mut ranges);
        if ranges.len() == 2 {
            println!("{}", (ranges[0].1 + 1) as u128 * rows as u128 + row as u128);
            break;
        }
    }
    Ok(())
}
