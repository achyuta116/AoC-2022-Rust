use std::error::Error;

struct Color(i16, i16, i16);

fn main() -> Result<(), Box<dyn Error>> {
    let fl = include_str!("base.txt");

    let fc = Color(0, 0, 255);
    let bc = Color(0, 255, 0);

    let mut line_count = 0;
    fl.lines().for_each(|_| line_count += 1);
    let ln = fl
        .lines()
        .map(|l| {
            l.chars()
                .enumerate()
                .map(|(i, c)| (i, c.is_alphanumeric()))
                .filter(|&(i, is_alphanumeric)| i == 0 || !is_alphanumeric)
                .count()
        })
        .max()
        .unwrap();

    let mut colored = vec![];
    let mut input = fl.lines();
    let mut line_no = 0;
    while let Some(line) = input.next() {
        let mut tmp = vec![];
        for (index, ch) in line.chars().enumerate() {
            let percent =
                (index as f64).hypot(line_no as f64) / (line_count as f64).hypot(ln as f64);
            let r: u8 = (fc.0 as f64 + percent * (bc.0 - fc.0) as f64) as u8;
            let g: u8 = (fc.1 as f64 + percent * (bc.1 - fc.1) as f64) as u8;
            let b: u8 = (fc.2 as f64 + percent * (bc.2 - fc.2) as f64) as u8;
            tmp.push(format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, ch));
        }
        colored.push(tmp);
        line_no += 1;
    }

    for v in colored {
        for s in v {
            print!("{}", s);
        }
        println!();
    }

    Ok(())
}
