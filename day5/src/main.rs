use aoc_util::{get_seed_data, str_strip_numbers};
use std::collections::VecDeque;
const PLACEHOLDER: &str = "[-]";
const EMPTYSPACE: &str = "    ";

#[derive(Debug,Copy, Clone)]
struct Instruction {
  count: i64,
  source: usize,
  dest: usize,
}

fn main() {
  let mut rows : Vec<String> = [].to_vec();
  let mut instructions : Vec<Instruction> = Vec::new();


  let input = get_seed_data().expect("Could not load values");

  let t = input.clone();
  // find the empty line as it is a good divider between the start and rules
  let x = t.iter().position(|r| r == "").unwrap();
  // use that to split the vector into two arrays
  let (icrates, rules) = t.split_at(x+1);
  let mut crates: Vec<String> = icrates.to_vec();
  // get rid of the two lines we don't care about
  crates.pop(); // get rid of blank row.
  crates.pop(); // get rid of index row.

  for line in crates {
    let i = line.to_owned();
    rows.push(i.replace(EMPTYSPACE,PLACEHOLDER).replace(" ",",").replace("][","],["));
  }
  let stacks : Vec<VecDeque<String>>  = transform_input_rows_to_stacks(&rows);

  for line in rules {
      let i = line.to_owned();
      let v: Vec<i64> =  str_strip_numbers(&i);
      instructions.push(build_instruction(v.clone()));
  }

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


fn transform_input_rows_to_stacks(rows: &Vec<String>) -> Vec<VecDeque<String>> {
  let mut temp_stacks : Vec<VecDeque<String>> = [].to_vec();
  let mut maxlen = 0;
  for x in rows {
    let y: Vec<&str> = x.split(",").collect();
    let foo = y.len();
    if  foo > maxlen {
      maxlen = foo
    }
  }
  let mut final_rows = vec!();
  for x in rows {
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