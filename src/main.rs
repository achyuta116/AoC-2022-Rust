use std::cmp::max;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt").unwrap();

    let mut trees = vec![];

    for line in input.lines() {
        let treeline = line.chars().collect::<Vec<_>>();
        trees.push(treeline);
    }

    let mut mx = 1;
    // l to r
    for i in 1..trees.len() - 1 {
        for j in 1..trees[0].len() - 1 {
            let mut sc = 1;
            let mut sc_mul = 0;
            let ht = trees[i][j].to_digit(10).unwrap();


            for k in j + 1..trees.len() {
                if trees[i][k].to_digit(10).unwrap() < ht {
                    sc_mul += 1;
                } else {
                    sc_mul += 1;
                    break
                }
            }
            sc *= sc_mul;

            sc_mul = 0;
            for k in (0..=j - 1).rev() {
                if trees[i][k].to_digit(10).unwrap() < ht {
                    sc_mul += 1;
                } else {
                    sc_mul += 1;
                    break
                }
            }
            sc *= sc_mul;

            sc_mul = 0;
            for k in i + 1..trees.len() {
                if trees[k][j].to_digit(10).unwrap() < ht {
                    sc_mul += 1;
                } else {
                    sc_mul += 1;
                    break
                }
            }
            sc *= sc_mul;

            sc_mul = 0;
            for k in (0..=i - 1).rev() {
                if trees[k][j].to_digit(10).unwrap() < ht {
                    sc_mul += 1;
                } else {
                    sc_mul += 1;
                    break
                }
            }
            sc *= sc_mul;

            mx = max(mx, sc);
        }
    }

    println!("{}", mx);

    Ok(())
}
