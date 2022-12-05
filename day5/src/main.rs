use aoc_util::{get_seed_data};
use std::collections::VecDeque;
const PLACEHOLDER: &str = "[-]";
const EMPTYSPACE: &str = "    ";

fn main() {
  let mut original_rows : Vec<String> = [].to_vec();
  let mut instructions : Vec<Vec<i32>> = [].to_vec();

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
      let ins = i.replace("move ","").replace(" from ", ",").replace(" to ", ",");
      let v: Vec<i32> = ins
      .split(",")
      .map(|s| s.parse().expect("parse error"))
      .collect();
      instructions.push(v);
    }


  }


  let mut maxlen = 0;
  for x in &original_rows {
    let y: Vec<&str> = x.split(",").collect();
    let foo = y.len();
    if  foo > maxlen {
      maxlen = foo
    }
  }
  let mut final_rows = vec!();
  for x in &original_rows {
    let mut y: Vec<&str> = x.split(",").collect();
    let mut foo = y.len();
    while foo < maxlen {
      y.push(PLACEHOLDER);
      foo += 1;
    }
    final_rows.push(y);
  }

  let mut temp_stacks : Vec<VecDeque<String>> = [].to_vec();

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
  println!("temp_stacks - {:?}", temp_stacks);



  for ins in instructions {
    // move 0 to 1 from 2
    let count = ins[0];
    let source: usize  = (ins[1]-1) as usize;
    let dest: usize = (ins[2]-1) as usize;

    for _i in 0..count {
      let val = temp_stacks[source].pop_back().unwrap();
      temp_stacks[dest].push_back(val);
    }
  }

  let mut msg: String = "".to_owned();
  for mut stack in temp_stacks {
    let bs: String = stack.pop_back().unwrap();
    //let b = bs.clone();
    msg.push_str(&bs);
  }


  println!("part1 - {:?}",msg);






}
