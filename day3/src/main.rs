use std::{
  env,
  fs::File,
  io::{self, BufRead, BufReader},
  path::Path,
  cmp::Ordering
};

fn main() -> std::io::Result<()> {



  let file_name = "/values.txt";
  let path = env::current_dir()?;
  let input_file = path.display().to_string() + file_name;
  let rucks = lines_from_file(input_file).expect("Could not load values");

  // put a underscore at the beginning so I don't have to do index math of +1 all the time
  // position in the string is the priority of the letter
  let keys = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

  let mut priority_total = 0;

  let mut first_ruck = "".to_owned();
  let mut second_ruck = "".to_owned();
  let mut group_ruck_count = 0;
  let mut group_priority_total = 0;
  for ruck in rucks {

    // PART 1
    let mid = ruck.len()/2;
    let (c1, c2) = ruck.split_at(mid);
    let c3 = common_char(c1,c2);
    let priority = keys.find(&c3).unwrap();
    priority_total += priority;

    // PART 2
    if first_ruck.eq("") {
      group_ruck_count = 1;
      first_ruck = ruck;
    } else {
      group_ruck_count += 1;
      if group_ruck_count == 2 {
        second_ruck = ruck;
      } else {  // 3
        let item = common_char_3(&first_ruck,&second_ruck, &ruck);
        let priority = keys.find(&item).unwrap();
        group_priority_total += priority;
        first_ruck = "".to_owned();
        second_ruck = "".to_owned();
        // println!("Round {}, Char: {}, Priority: {}, Total: {}", r / 3, item, priority, group_priority_total);
      }
    }

  }
  println!("Round 1 Result = {}", priority_total);
  println!("Round 2 Result = {}", group_priority_total);

  Ok(())
}

fn common_char(a: &str, b: &str) -> String {
  let mut v: Vec<String> = vec![a.into(), b.into()];
  v = sort_string_vector_by_string_len(&v);

  for c in v[0].chars() {
    if v[1].contains(c) {
      return c.to_string();
    }
  }
  return "".to_string();

}
// can't overload so I have a dumb name for a 3 string version.
fn common_char_3(a: &str, b: &str, c: &str) -> String {

 let mut v: Vec<String> = vec![a.into(), b.into(), c.into()];
 v = sort_string_vector_by_string_len(&v);


  for c in v[0].chars() {
    if v[1].contains(c) && v[2].contains(c) {
      return c.to_string();
    }
  }
  return "".to_string();

}

fn sort_string_vector_by_string_len(v: &[String]) -> Vec<String> {
  let mut vec: Vec<String> = v.to_vec();
  vec.sort_by(|a, b| {
    if a.len() < b.len() {
        Ordering::Less
    } else if a.len() == b.len() {
        Ordering::Equal
    } else {
        Ordering::Greater
    }
  });

  return vec;

}

// function found at https://stackoverflow.com/a/35820003
fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
  BufReader::new(File::open(filename)?).lines().collect()
}
