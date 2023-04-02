use std::error::Error;
use std::fs::read_to_string;

#[derive(Debug)]
struct Monkey {
    items: Vec<i32>,
    operation: char,
    operand: Option<i32>,
    modulus: i32,
    on_true: usize,
    on_false: usize,
    inspection: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt").unwrap();
    let mut input = input.lines();
    let mut monkeys = vec![];

    while let Some(mut line) = input.next() {
        line = line.strip_prefix("Monkey ").unwrap();
        line = line.strip_suffix(":").unwrap();
        let _ = line.parse::<i32>().unwrap();

        let mut line = input.next().unwrap();
        let items = line
            .trim()
            .strip_prefix("Starting items: ")
            .unwrap()
            .split(", ")
            .collect::<Vec<_>>();
        let items = items.iter().map(|&x| x.parse::<i32>().unwrap()).collect();
        line = input.next().unwrap();
        line = line.trim().strip_prefix("Operation: new = old ").unwrap();
        let (first, last) = line.split_once(" ").unwrap();

        line = input.next().unwrap();
        let modulus = line
            .trim()
            .strip_prefix("Test: divisible by ")
            .unwrap()
            .parse::<i32>()
            .unwrap();

        line = input.next().unwrap();
        let on_true = line
            .trim()
            .strip_prefix("If true: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        line = input.next().unwrap();

        let on_false = line
            .trim()
            .strip_prefix("If false: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        monkeys.push(Monkey {
            items,
            operation: first.chars().next().unwrap(),
            modulus,
            operand: match last.parse() {
                Ok(i) => Some(i),
                Err(_) => None,
            },
            on_true,
            on_false,
            inspection: 0
        });

        input.next();
    }

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop() {
                monkeys[i].inspection += 1;
                let operand = match monkeys[i].operand {
                    Some(value) => value,
                    None => item,
                };

                let mut new = match monkeys[i].operation {
                    '*' => item * operand,
                    '+' => item + operand,
                    _ => panic!("Incorrect Operation")
                };

                new /= 3;
                
                if new % monkeys[i].modulus == 0 {
                    let throw = monkeys[i].on_true;
                    monkeys[throw].items.push(new);
                } else {
                    let throw = monkeys[i].on_false;
                    monkeys[throw].items.push(new);
                }
            }
        }
    }
    
    let mut inspections = monkeys.iter().map(|x| x.inspection).collect::<Vec<_>>();
    inspections.sort();
    inspections.reverse();
    println!("{}", inspections[0] * inspections[1]);

    Ok(())
}
