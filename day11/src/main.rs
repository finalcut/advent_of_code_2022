use aoc_util::{read_file, split_delimited, str_strip_numbers};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Monkey {
    name: String,
    worry: VecDeque<i64>,
    operation: String,
    adjustment: String,
    divisor: i64,
    pass: usize,
    fail: usize,
    items_inspected: i64,
}

fn main() {
    let observations: Vec<String> = read_file("values.txt").expect("Could not load values");
    let monkey_info = split_delimited(&observations, &"".to_owned());
    let boredom_factor: i64 = 3;

    let mut monkeys: Vec<Monkey> = Vec::new();

    for info in monkey_info {
        monkeys.push(get_monkey(&info.to_vec()));
    }

    let total_rounds = 20;

    for _i in 0..20 {
        //let mut monkey_pass = monkeys.clone();
        for x in 0..monkeys.len() {
            while monkeys[x].worry.len() > 0 {
                monkeys[x].items_inspected += 1;
                let worry: i64 = monkeys[x].worry.pop_front().unwrap();
                let adj: i64 = if monkeys[x].adjustment.eq("old") {
                    worry
                } else {
                    monkeys[x].adjustment.parse::<i64>().unwrap()
                };
                let new_worry = compute_worry(&monkeys[x].operation, worry, adj, boredom_factor);

                let mut index_to_update = 0;
                if (new_worry % monkeys[x].divisor) == 0 {
                    // success condition
                    index_to_update = monkeys[x].pass;
                } else {
                    index_to_update = monkeys[x].fail;
                }
                let _ = monkeys[index_to_update].worry.push_back(new_worry);
            }
        }
    }

    monkeys.sort_by(|a, b| b.items_inspected.cmp(&a.items_inspected));
    let part1 = monkeys[0].items_inspected * monkeys[1].items_inspected;
    println!("part 1: {}", part1);
}

fn compute_worry(operation: &str, worry: i64, adjustment: i64, divisor: i64) -> i64 {
    let temp_worry: i64 = match operation {
        "*" => worry * adjustment,
        "/" => worry / adjustment,
        "+" => worry + adjustment,
        "-" => worry - adjustment,
        _ => worry,
    };

    if divisor != 0 {
        return temp_worry / divisor;
    } else {
        return temp_worry;
    }
}

fn get_monkey(info: &Vec<String>) -> Monkey {
    let i = info[3].clone();
    let mut move_info: Vec<&str> = i.split_whitespace().collect();
    let adj: String = move_info.pop().unwrap().to_string();
    let op: String = move_info.pop().unwrap().to_string();

    let worry = VecDeque::from_iter(str_strip_numbers(&info[2]));

    Monkey {
        name: info[1].clone(),
        worry: worry,
        operation: op,
        adjustment: adj,
        divisor: str_strip_numbers(&info[4])[0],
        pass: str_strip_numbers(&info[5])[0] as usize,
        fail: str_strip_numbers(&info[6])[0] as usize,
        items_inspected: 0,
    }
}
