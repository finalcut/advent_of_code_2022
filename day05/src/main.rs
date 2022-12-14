use aoc_util::{get_seed_data, str_strip_numbers, string_to_vector_by_len};

use std::collections::VecDeque;

#[derive(Debug, Copy, Clone)]
struct Instruction {
    count: i64,
    source: usize,
    dest: usize,
}

fn main() {
    let input = get_seed_data().expect("Could not load values");

    // find the empty line as it is a good divider between the start and rules
    let x = input.iter().position(|r| r == "").unwrap();
    // use that to split the vector into two arrays
    let (icrates, rules) = input.split_at(x + 1);
    let crates = remove_index_and_blank_row(icrates);

    let stacks: Vec<VecDeque<String>> = transform_input_rows_to_stacks(&crates);
    let instructions = get_instructions(&rules);

    let result1 = crane_mover_9000(&instructions, stacks.clone());
    show_message("part1".to_owned(), result1);

    let result2 = crane_mover_9001(&instructions, stacks.clone());
    show_message("part2".to_owned(), result2);
}

fn get_instructions(rules: &[String]) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in rules {
        let i = line.to_owned();
        let v: Vec<i64> = str_strip_numbers(&i);
        instructions.push(build_instruction(v.clone()));
    }
    return instructions;
}

fn remove_index_and_blank_row(input: &[String]) -> Vec<String> {
    let mut crates: Vec<String> = input.to_vec();
    // get rid of the two lines we don't care about
    crates.pop(); // get rid of blank row.
    crates.pop(); // get rid of index row.
    return crates;
}

fn get_message_from_stacks(stacks: Vec<VecDeque<String>>) -> String {
    let mut msg: String = "".to_owned();
    for mut stack in stacks {
        let bs: String = stack.pop_back().unwrap(); // build the message from the "top" of each stack
                                                    //let b = bs.clone();
        msg.push_str(&bs);
    }
    msg = msg.replace("[", "").replace("]", "");
    return msg;
}

fn show_message(caption: String, result: String) {
    println!("{:?} - {:?}", caption, result);
}

fn transform_input_rows_to_stacks(rows: &Vec<String>) -> Vec<VecDeque<String>> {
    let mut stacks: Vec<VecDeque<String>> = [].to_vec();
    let mut final_rows = vec![];

    for x in rows {
        let y: Vec<&str> = string_to_vector_by_len(&x, 4);
        final_rows.push(y);
    }

    for _n in 0..final_rows[0].len() {
        let stack: VecDeque<String> = VecDeque::new();
        stacks.push(stack);
    }

    for row in final_rows {
        for i in 0..row.len() {
            let cell = row[i].to_string().trim().to_string();
            if cell.len() > 0 {
                stacks[i].push_front(cell);
            }
        }
    }
    return stacks;
}

// stole function naming idea from: https://philip-weinke.de/2022/12/advent-of-rust-5/
fn crane_mover_9000(instructions: &Vec<Instruction>, mut stacks: Vec<VecDeque<String>>) -> String {
    for ins in instructions {
        for _i in 0..ins.count {
            let val = stacks[ins.source].pop_back().unwrap();
            stacks[ins.dest].push_back(val);
        }
    }

    return get_message_from_stacks(stacks);
}

fn crane_mover_9001(instructions: &Vec<Instruction>, mut stacks: Vec<VecDeque<String>>) -> String {
    for ins in instructions {
        //println!("ins: {:?}", ins);
        let mut claw: VecDeque<String> = VecDeque::new();
        for _i in 0..ins.count {
            let val = stacks[ins.source].pop_back().unwrap();
            claw.push_front(val);
        }

        while claw.len() > 0 {
            let con = claw.pop_front().unwrap();
            stacks[ins.dest].push_back(con.to_string());
        }
    }
    return get_message_from_stacks(stacks);
}

fn build_instruction(ins: Vec<i64>) -> Instruction {
    Instruction {
        count: ins[0],
        // this is a stupid place to do this probably..good luck debugging if shit hits the fan
        source: (ins[1] - 1) as usize,
        dest: (ins[2] - 1) as usize,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_util::get_test_data;

    #[test]
    fn test_input_parsing_test() {
        let input = get_test_data().expect("Could not load values");

        // find the empty line as it is a good divider between the start and rules
        let x = input.iter().position(|r| r == "").unwrap();
        // use that to split the vector into two arrays
        let (icrates, rules) = input.split_at(x + 1);
        let crates = remove_index_and_blank_row(icrates);

        let stacks: Vec<VecDeque<String>> = transform_input_rows_to_stacks(&crates);
        let instructions = get_instructions(&rules);

        let result1 = crane_mover_9000(&instructions, stacks.clone());
        assert_eq!(result1, "CMZ");

        let result2 = crane_mover_9001(&instructions, stacks.clone());
        assert_eq!(result2, "MCD");
    }
}
