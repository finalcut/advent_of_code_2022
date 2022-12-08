use aoc_util::{get_seed_data, str_strip_numbers};
use std::collections::HashMap;

fn main() {
  let input = get_seed_data().expect("Could not load values");


  //let mut parent_dir = &mut current_node;
  let info = build_path_map(input);


  //println!("info : {:#?}", info);

  let answer1 = sum_directories_bigger_than(&info, 100000 as i64);
  println!("answer 1 : {:#?}", answer1);


  // max space = 70000000
  // need free space of 30000000
  let key = ",/";
  let keytotal = get_dir_size(&info, key);
  let freespace = 70000000 - keytotal;
  let needed = 30000000 - freespace;
  let answer2 = smallest_dir_size_bigger_than(&info, needed, freespace);
  println!("answer 2 : {:#?}", answer2);
}

fn build_path_map(input: Vec<String>) -> HashMap::<String, i64> {
  let mut current_key : String = "".to_string();
  let mut info : HashMap<String, i64> = HashMap::<String, i64>::new();
  for line in input.iter() {
    let parts : Vec<&str> = line.split_whitespace().collect();

    if parts[1].trim().eq("cd") {

      let name = parts[2].trim();

      if !name.eq("..") {
        let mut keyval = current_key.clone();
        keyval = keyval + "," + name;
        current_key = keyval.clone();
        info.insert(keyval.clone(), 0 as i64);

       //let idx = current_node.dirs.iter().position(|r| r.name.eq(&name)).unwrap();
        //println!("CURRENT : {:#?}", idx);
        //parent_dir = &mut current_node;
        //current_node = &mut current_node.dirs[idx];
      } else {
        // remove the last key from the current_key
        let key = current_key.clone();
        let (t, _x) = key.rsplit_once(',').unwrap();
        current_key = t.to_string().clone();

        //
      }
    }

    if parts[1].trim().eq("ls") {
      continue;
    }


    // FOUND A FILE; put it in the current DIR
    if line.chars().next().unwrap().is_digit(10) {
      let size = str_strip_numbers(line);

     // println!("key: {}, Total: {}", current_key, size[0]);

      if let Some(x) = info.get_mut(&current_key) {
        *x = *x + size[0];
      }
    }

    if parts[0].trim().eq("dir") {
      // dont care
    }

  }
  return info;
}

fn sum_directories_bigger_than(data: &HashMap<String, i64>, max_size: i64) -> i64 {

  let mut total : i64 = 0;


  for key in data.keys() {
    let keytotal = get_dir_size(&data, &key);

    if keytotal <= max_size {
      total = total + keytotal.clone();
    }
  }

  return total;
}

fn smallest_dir_size_bigger_than(data: &HashMap<String, i64>, min_size: i64, max_size: i64) -> i64 {
  let mut ans : i64 = max_size;

  for key in data.keys() {
    let keytotal = get_dir_size(&data, &key);

    if keytotal > min_size && keytotal < ans {
      ans = keytotal.clone();
    }

  }

  return ans;
}

fn get_dir_size(data: &HashMap<String, i64>, key: &str) -> i64 {
  let mut keytotal : i64 = 0;
  for ikey in data.keys() {
    if ikey.contains(key) {
      let val = data.get(ikey).unwrap();
      //println!("key: {}, ikey: {}, val: {}", key, ikey, val);

      keytotal = keytotal + val.clone();
    }
  }
  return keytotal;
}