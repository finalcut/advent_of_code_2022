use aoc_util::{get_seed_data, str_strip_numbers};
use std::collections::VecDeque;
const PLACEHOLDER: &str = "[-]";
const EMPTYSPACE: &str = "    ";

#[derive(Copy, Clone)]
struct Instruction {
  count: i64,
  source: usize,
  dest: usize,
}

fn main() {
  let mut original_rows : Vec<String> = [].to_vec();
  let mut instructions : Vec<Instruction> = Vec::new();

  let mut mode: i16 = 0;

  let input = get_seed_data().expect("Could not load values");

  for line in input {
    let i = line.to_owned();

    if mode == 1 {
      mode = 2;
    }

    if i.trim().len() == 0 || (i.contains("1") && !i.contains("move")) {
      mode = 1; // skip line
    }
    if mode == 0 {
      original_rows.push(i.replace(EMPTYSPACE,PLACEHOLDER).replace(" ",",").replace("][","],["));
    }

    if mode == 2 {
      let v: Vec<i64> =  str_strip_numbers(&i);
      instructions.push(build_instruction(v.clone()));
    }
  }

  let stacks : Vec<VecDeque<String>>  = transform_input_rows_to_stacks(&original_rows);

  crane_mover_9000(&instructions, stacks.clone());

  crane_mover_9001(&instructions, stacks.clone());

}

fn show_message(caption: String, stacks: Vec<VecDeque<String>>) {
  let mut msg: String = "".to_owned();
  for mut stack in stacks {
    let bs: String = stack.pop_back().unwrap();
    //let b = bs.clone();
    msg.push_str(&bs);
  }
  msg = msg.replace("[","").replace("]","");
  println!("{:?} - {:?}",caption, msg);

}


fn transform_input_rows_to_stacks(original_rows: &Vec<String>) -> Vec<VecDeque<String>> {
  let mut temp_stacks : Vec<VecDeque<String>> = [].to_vec();
  let mut maxlen = 0;
  for x in original_rows {
    let y: Vec<&str> = x.split(",").collect();
    let foo = y.len();
    if  foo > maxlen {
      maxlen = foo
    }
  }
  let mut final_rows = vec!();
  for x in original_rows {
    let mut y: Vec<&str> = x.split(",").collect();
    let mut foo = y.len();
    while foo < maxlen {
      y.push(PLACEHOLDER);
      foo += 1;
    }
    final_rows.push(y);
  }


  for _n in 0..final_rows[0].len() {
    let stack : VecDeque<String> = VecDeque::new();
    temp_stacks.push(stack);
  }

  for row in final_rows {
    for i in 0..row.len() {
      let val = row[i].to_string();
      if !val.eq(PLACEHOLDER) {
        temp_stacks[i].push_front(row[i].to_string());
      }
    }
  }
  return temp_stacks;
}

// stole function naming idea from: https://philip-weinke.de/2022/12/advent-of-rust-5/
fn crane_mover_9000(instructions: &Vec<Instruction>, mut stacks: Vec<VecDeque<String>>){
  for ins in instructions {


    for _i in 0..ins.count {
      let val = stacks[ins.source].pop_back().unwrap();
      stacks[ins.dest].push_back(val);
    }
  }


  show_message("part1".to_owned(), stacks);
}

fn crane_mover_9001(instructions: &Vec<Instruction>, mut stacks: Vec<VecDeque<String>>){
  for ins in instructions {

    //println!("ins: {:?}", ins);
    let mut claw : VecDeque<String> =  VecDeque::new();
    for _i in 0..ins.count {

      let val = stacks[ins.source].pop_back().unwrap();
      claw.push_front(val);
    }


    while claw.len() > 0 {
      let con = claw.pop_front().unwrap();
      stacks[ins.dest].push_back(con.to_string());

    }

  }
  show_message("part2".to_owned(), stacks);
}

fn build_instruction(ins: Vec<i64>) -> Instruction {
  Instruction {
    count: ins[0],
    // this is a stupid place to do this probably..good luck debugging if shit hits the fan
    source: (ins[1] - 1) as usize,
    dest: (ins[2]-1) as usize,
  }
}