/*
  Notes:
    numbers as single character strings such as "6" and "5" have standard comparison operations like < and > and act just like their numeric counterparts

failure: 1646 too small
*/
fn main() {
    let input: Vec<Vec<&str>> = include_str!("../values.txt")
        .split("\n\n")
        .map(|block| block.trim())
        .map(|block| { block.split("\n").collect() })
        .collect();

    let mut part1_good_indexes: Vec<usize> = [].to_vec();
    for c in 0..input.len() {
        let x = json::parse(input[c][0]).unwrap();
        let y = json::parse(input[c][1]).unwrap();


        if compare(x,y) == 1 {
          part1_good_indexes.push(c + 1);
        }
    }

    let mut flat : Vec<&str> = input.concat();
    flat.push("[[2]]");
    flat.push("[[6]]");
   // flat.sort_by(|a,b| compare(a,b) );


    let sum: usize = part1_good_indexes.iter().sum();
    println!("part 1: {:?}", sum);
    println!("indexes of success {:?}", part1_good_indexes);
}

fn compare(x: json::JsonValue, y: json::JsonValue) -> i16 {
    out_fails("x/y", x.clone(), y.clone());


    if x.is_array() && y.is_array() {
      let mut max = x.len();
      if y.len() > max {
        max = y.len();
      }

      for i in 0..max {
        if i >= x.len() {return 1};
        if i >= y.len() {return -1};

        let result = compare(x[i].clone() ,y[i].clone());
        if result != 0 {
          return result;
        }

      }

      return 0; // they are the same..

  } else if x.is_number() & y.is_number() {
    if x.as_i16().unwrap() > y.as_i16().unwrap() { return -1 };
    if y.as_i16().unwrap() > x.as_i16().unwrap() { return 1 };
    return 0; // they are the same
  } else {
    if x.is_array() {
      return compare(x, json::array![y.as_i16().unwrap()]);
    } else {
      return compare(json::array![x.as_i16().unwrap()] , y)
    }
  }



}

fn out_fails(m: &str, a: json::JsonValue, b: json::JsonValue) {
    println!("msg: {}",  m);
    println!("{} {:?}", a.len(), json::stringify(a.clone()));
    println!("{} {:?}", b.len(), json::stringify(b.clone()));
    println!();
}