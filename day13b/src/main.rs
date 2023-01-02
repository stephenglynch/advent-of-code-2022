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

    // Parse into 'packets' (JSON objects) excluding empty lines
    let contents = fs::read_to_string("input/input.txt").unwrap();
    let lines: Vec<&str> = contents.lines().filter(|x| !x.is_empty()).collect();
    let mut packets: Vec<Value> = lines.iter().map(|x| serde_json::from_str(x).unwrap()).collect();

    // Add dividers
    let div1 = serde_json::to_value(vec![vec![2]]).unwrap();
    let div2 = serde_json::to_value(vec![vec![6]]).unwrap();
    packets.push(div1.clone());
    packets.push(div2.clone());

    // Sort packets
    packets.sort_by(|l, r| {
        match compare(l, r) {
            Order::Correct => Ordering::Less,
            Order::Match => Ordering::Equal,
            Order::Incorrect => Ordering::Greater
        }
    });

    let ind1 = packets.iter().position(|x| x == &div1).unwrap() + 1;
    let ind2 = packets.iter().position(|x| x == &div2).unwrap() + 1;

    println!("answer: {:?}", ind1 * ind2);
}
