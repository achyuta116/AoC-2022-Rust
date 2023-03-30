use std::error::Error;
use std::fs::read_to_string;

#[derive(Clone, Copy)]
struct Point(i32, i32);

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt").unwrap();
    let mut input = input.lines();

    let mut i = 0;
    let mut x = 1;
    let mut line = vec![];

    while i < 240 {
        let mut instr = input.next().unwrap().split(" ");
        let first = instr.next().unwrap().to_owned();

        match first.as_str() {
            "noop" => {
                if (i % 40 - x as i32).abs() <= 1 {
                    line.push('#');
                } else {
                    line.push('.');
                }
                i += 1;
            }
            "addx" => {
                let second = instr.next().unwrap().parse::<i32>().unwrap();
                if (i % 40 - x as i32).abs() <= 1 {
                    line.push('#');
                } else {
                    line.push('.');
                }

                i += 1;

                if (i % 40 - x as i32).abs() <= 1 {
                    line.push('#');
                } else {
                    line.push('.');
                }
                i += 1;

                x += second;
            }
            _ => panic!("Invalid instr!"),
        }
    }

    for i in 0..6 {
        for j in 0..40 {
            print!("{}", line[i * 40 + j])
        }

        println!()
    }

    Ok(())
}
