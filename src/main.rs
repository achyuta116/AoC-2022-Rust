use std::error::Error;
use std::fs::read_to_string;

#[derive(Clone, Copy)]
struct Point(i32, i32);

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt").unwrap();
    let mut input = input.lines();

    let mut i = 0;
    let mut x = 1;
    let mut sum = 0;
    let mut mark = 20;

    while i <= 220 {
        let old_x = x;
        let mut instr = input.next().unwrap().split(" ");
        let first = instr.next().unwrap().to_owned();

        match first.as_str() {
            "noop" => {
                i += 1;
            }
            "addx" => {
                let second = instr.next().unwrap().parse::<i32>().unwrap();
                x += second;
                i += 2;
            }
            _ => panic!("Invalid instr!"),
        }

        if i > mark {
            sum += old_x * mark;
            mark += 40;
        } else if i == mark {
            sum += x * mark;
            mark += 40;
        }
    }

    println!("{}", sum);

    Ok(())
}
