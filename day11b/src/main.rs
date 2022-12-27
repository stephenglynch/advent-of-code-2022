use std::fs;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref MONKEY_PATTERN: Regex = Regex::new(std::concat!(
        r"Monkey (\d+):\n",
        r"  Starting items: ((\d+,? ?)+)\n",
        r"  Operation: new = old ([\*\+]) (\d+|old)\n",
        r"  Test: divisible by (\d+)\n",
        r"    If true: throw to monkey (\d+)\n",
        r"    If false: throw to monkey (\d+)"
    )).unwrap();
}

#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    op_type: String,
    op_num: u64,
    test: u64,
    true_monkey: usize,
    false_monkey: usize,
    inspections: usize
}

fn parse_items(text: &str) -> Vec<u64> {
    text.split(", ").map(|x| x.parse().unwrap()).collect()
}

fn parse_monkeys(text: &str) -> Vec<Monkey> {

    let mut monkeys = vec![];

    for cap in MONKEY_PATTERN.captures_iter(text) {
        let items = &cap[2];
        let op_type = &cap[4];
        let op_num = &cap[5];
        let test = &cap[6];
        let true_monkey = &cap[7];
        let false_monkey = &cap[8];

        let op_type = match op_num {
            "old" => "^",
            _ => op_type
        };

        let op_num: u64 = match op_num {
            "old" => 0,
            _ => op_num.parse().unwrap()
        };

        monkeys.push(Monkey {
            items: parse_items(&items),
            op_type: op_type.to_string(),
            op_num: op_num,
            test: test.parse().unwrap(),
            true_monkey: true_monkey.parse().unwrap(),
            false_monkey: false_monkey.parse().unwrap(),
            inspections: 0
        });
    }

    return monkeys;
}

fn calc_mod_value(monkeys: &Vec<Monkey>) -> u64 {
    let mut val = 1;
    for m in monkeys {
        val *= m.test;
    }
    return val;
}

fn run_round(mut monkeys: Vec<Monkey>, mod_val: u64) -> Vec<Monkey> {
    for i in 0..monkeys.len() {
        let mut true_items = vec![];
        let mut false_items = vec![];
        for item in monkeys[i].items.iter() {
            // Pop item and apply operations
            let item = *item;
            let item = match monkeys[i].op_type.as_ref() {
                "*" => (item * monkeys[i].op_num),
                "+" => (item + monkeys[i].op_num),
                "^" => (item * item),
                _ => panic!()
            };
            let item = item % mod_val;

            // Test and move item to next monkey
            if item % monkeys[i].test == 0 {
                true_items.push(item)
            } else {
                false_items.push(item)
            }
        }

        monkeys[i].inspections += monkeys[i].items.len();
        monkeys[i].items.clear();
        let true_monkey = monkeys[i].true_monkey;
        let false_monkey = monkeys[i].false_monkey;
        monkeys[true_monkey].items.append(&mut true_items);
        monkeys[false_monkey].items.append(&mut false_items);
    }
    return monkeys;
}

fn main() {
    let contents = fs::read_to_string("input/input.txt").unwrap();
    let mut monkeys = parse_monkeys(&contents);
    let mod_val = calc_mod_value(&monkeys);

    // run rounds
    for _ in 0..10000 {
        monkeys = run_round(monkeys, mod_val);
    }

    // collect inspection numbers
    let mut inspections: Vec<_> = monkeys.iter().map(|x| x.inspections).collect();
    for (i, n) in inspections.iter().enumerate() {
        println!("Monkey {} inspected items {} times.", i, n)
    }

    // print answer
    inspections.sort();
    inspections.reverse();
    println!("answer = {}", inspections[0] * inspections[1]);
}
