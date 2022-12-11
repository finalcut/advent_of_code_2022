use aoc_util::{read_file, split_delimited, get_i128_numbers_from_string};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Monkey {
    name: String,
    worry: VecDeque<i128>,
    operation: String,
    adjustment: String,
    divisor: i128,
    pass: usize,
    fail: usize,
    items_inspected: i128,
}

fn main() {
    let observations: Vec<String> = read_file("values.txt").expect("Could not load values");
    let monkey_info = split_delimited(&observations, &"".to_owned());
    let boredom_factor: i128 = 3;

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
                let worry: i128 = monkeys[x].worry.pop_front().unwrap();
                let adj: i128 = if monkeys[x].adjustment.eq("old") {
                    worry
                } else {
                    monkeys[x].adjustment.parse::<i128>().unwrap()
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

fn compute_worry(operation: &str, worry: i128, adjustment: i128, divisor: i128) -> i128 {
    let temp_worry: i128 = match operation {
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

    let worry = VecDeque::from_iter(get_i128_numbers_from_string(&info[2]));

    Monkey {
        name: info[1].clone(),
        worry: worry,
        operation: op,
        adjustment: adj,
        divisor: get_i128_numbers_from_string(&info[4])[0],
        pass: get_i128_numbers_from_string(&info[5])[0] as usize,
        fail: get_i128_numbers_from_string(&info[6])[0] as usize,
        items_inspected: 0,
    }
}
