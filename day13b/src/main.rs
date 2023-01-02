use std::cmp::Ordering;
use std::fs;
use std::iter;
use serde_json::Value;

#[derive(Debug)]
enum Order {
    Match,
    Correct,
    Incorrect
}

fn compare(left: &Value, right: &Value) -> Order {
    match (left, right) {
        (Value::Number(ln), Value::Number(rn)) => {
            let ln = ln.as_u64().unwrap();
            let rn = rn.as_u64().unwrap();
            if ln < rn {
                return Order::Correct
            } else if ln == rn {
                return Order::Match
            } else {
                return Order::Incorrect
            }
        },
        (Value::Array(larr), Value::Array(rarr))  => {
            for (lval, rval) in iter::zip(larr, rarr) {
                match compare(lval, rval) {
                    Order::Correct => return Order::Correct,
                    Order::Incorrect => return Order::Incorrect,
                    Order::Match => ()
                }
            }
            return match larr.len().cmp(&rarr.len()) {
                Ordering::Equal => Order::Match,
                Ordering::Less => Order::Correct,
                Ordering::Greater => Order::Incorrect
            }
        },
        (larr @ Value::Array(_), Value::Number(rn)) => {
            let rarr = serde_json::to_value(vec![rn]).unwrap();
            return compare(&larr, &rarr);
        },
        (Value::Number(ln), rarr @ Value::Array(_)) => {
            let larr = serde_json::to_value(vec![ln]).unwrap();
            return compare(&larr, &rarr);
        }
        _ => panic!()
    }
}

fn main() {
    let contents = fs::read_to_string("input/input.txt").unwrap();

    let lines: Vec<&str> = contents.lines().collect();

    let mut answer = 0;
    for (ind, group) in lines.chunks(3).enumerate() {
        let first: Value = serde_json::from_str(group[0]).unwrap();
        let second: Value = serde_json::from_str(group[1]).unwrap();
        let cmp = compare(&first, &second);
        match cmp {
            Order::Correct => answer += ind + 1,
            _ => ()
        }
        println!();
    }

    println!("answer: {}", answer);
}
