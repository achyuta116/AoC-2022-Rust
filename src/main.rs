use std::cmp::Ordering;
use std::error::Error;
use std::fs::read_to_string;
use std::str::Chars;

#[derive(Debug)]
enum ListItem {
    Integer(i32),
    List(Vec<ListItem>),
}

fn parse_list_item(item: &mut Chars) -> ListItem {
    let mut ret = vec![];
    while let Some(ch) = item.next() {
        match ch {
            '0'..='9' => {
                let mut number = String::from(ch);
                while let Some(ch) = item.next() {
                    match ch {
                        ',' => {
                            ret.push(ListItem::Integer(number.parse::<i32>().unwrap()));
                            break;
                        }
                        ']' => {
                            ret.push(ListItem::Integer(number.parse::<i32>().unwrap()));
                            return ListItem::List(ret);
                        }
                        '0'..='9' => number.push(ch),
                        _ => panic!("Invalid Character"),
                    }
                }
            }
            '[' => ret.push(parse_list_item(item)),
            ']' => break,
            ',' => continue,
            _ => panic!("Invalid character"),
        }
    }
    ListItem::List(ret)
}

fn compare_list_items(left: &ListItem, right: &ListItem) -> Option<bool> {
    match (left, right) {
        (ListItem::Integer(l), ListItem::Integer(r)) => {
            if l == r {
                None
            } else {
                Some(l < r)
            }
        }
        (ListItem::Integer(l), ListItem::List(_)) => compare_list_items(
            &ListItem::List(vec![ListItem::Integer(l.to_owned())]),
            right,
        ),
        (ListItem::List(_), ListItem::Integer(r)) => {
            compare_list_items(left, &ListItem::List(vec![ListItem::Integer(r.to_owned())]))
        }
        (ListItem::List(l), ListItem::List(r)) => {
            if l.is_empty() && r.is_empty() {
                return None;
            } else if l.is_empty() {
                return Some(true);
            } else if r.is_empty() {
                return Some(false);
            }

            let mut l = l.iter();
            let mut r = r.iter();
            loop {
                match (l.next(), r.next()) {
                    (Some(left), Some(right)) => {
                        if let Some(correct) = compare_list_items(left, right) {
                            return Some(correct);
                        }
                    }
                    (Some(_), None) => return Some(false),
                    (None, Some(_)) => return Some(true),
                    (None, None) => return None,
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt").unwrap();
    let mut input = input.lines();

    let mut messages = vec![];
    let divider_two = ListItem::List(vec![ListItem::List(vec![ListItem::Integer(2)])]);
    let divider_six = ListItem::List(vec![ListItem::List(vec![ListItem::Integer(6)])]);
    messages.push(divider_two);
    messages.push(divider_six);
    while let Some(line) = input.next() {
        let left = parse_list_item(&mut line.strip_prefix("[").unwrap().chars());
        let right = parse_list_item(&mut input.next().unwrap().strip_prefix("[").unwrap().chars());
        input.next();
        messages.push(left);
        messages.push(right);
    }

    messages.sort_by(|left, right| match compare_list_items(left, right) {
        Some(true) => Ordering::Less,
        Some(false) => Ordering::Greater,
        None => Ordering::Equal,
    });

    let two_index = messages
        .iter()
        .position(|r| {
            compare_list_items(
                r,
                &ListItem::List(vec![ListItem::List(vec![ListItem::Integer(2)])]),
            )
            .is_none()
        })
        .unwrap();
    let six_index = messages
        .iter()
        .position(|r| {
            compare_list_items(
                r,
                &ListItem::List(vec![ListItem::List(vec![ListItem::Integer(6)])]),
            )
            .is_none()
        })
        .unwrap();

    println!("{}", (two_index + 1) * (six_index + 1));

    Ok(())
}
