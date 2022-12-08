use aoc_util::{read_file};
fn main() {
    let input = read_file("values.txt").expect("Could not load values");

    let ncols: usize = input[0].len();

    let mut data = Vec::new();
    let mut nrows = 0;
    for line in input {
      let x : Vec<u32> = line.chars().flat_map(|x| x.to_digit(10)).collect();
      data.push(x);
        // Compute `row` and append it to `data`; this is a trivial example:
       // data.push(line.chars().flat_map(|x| x.to_string().parse()).collect() as Vec<i32>);
        nrows += 1;
    }

    let mut tall_trees = 0;
    let mut max_scienic_score: i64 = 0;
    let mut big_x = 0;
    let mut big_y = 0;
    for row_id in 1..(nrows -1) {
      for col_id in 1..(ncols -1) {
          // after this split the right array will contain our val as the first value.
          let (left, right) = data[row_id].split_at(col_id);
          let val: u32 = data[row_id][col_id];
          let mut tall = false;
          if is_tallest_in_row(val, left, right) {
            tall = true;
      //      println!("_ROW val: {}, at: {},{}", val, row_id, col_id);
          }

          if is_tallest_in_col(val, &data, row_id, col_id){
            tall = true;
            // println!("tall tree in column! {} at:  {},{}", val, row_id, col_id);
          }

          let tree_score = get_tree_score(&data, row_id, col_id);
          if tree_score > max_scienic_score {
            max_scienic_score = tree_score;
            big_y = col_id;
            big_x = row_id;
          }

          if tall {
            tall_trees +=1;
          }

      }
    }

    let border_trees = (nrows*2) + ((ncols-2)*2);


    let total = tall_trees + border_trees;

    println!("border_trees: {:?}, tall_trees: {:?}, total_visible: {:?}", border_trees, tall_trees, total);
    println!("max score {} at:  {},{}", max_scienic_score, big_x, big_y);

}

fn is_tallest_in_row(v: u32, left: &[u32], right: &[u32]) -> bool {
  let mut left_tall : bool = true;
  let mut right_tall: bool = true;
  for i in left {
    if v <= *i {
      left_tall = false;
    }
  }

  for i in right.iter().skip(1){
    if v <= *i {
      right_tall = false;
    }

  }
  //println!("v: {}, left: {:?}, right: {:?}, tall: {:?}", v, left, right, left_tall || right_tall);
  return left_tall || right_tall;
}

fn is_tallest_in_col(val: u32, data: &Vec<Vec<u32>>, row_index: usize, col_index: usize) -> bool {

    let mut is_tallest_a = true;
    let mut is_tallest_b: bool = true;

    let mut i = 0;

    for row in data{
      if i < row_index {
        let cmp = row[col_index];
        if val <= cmp  {
          is_tallest_a = false;
          break;
        }
      }
      i += 1;
    }
    if is_tallest_a {
    //  println!("ACOL val: {}, at: {},{}", val, row_index, col_index);
    }



      i = 0;


      for row in data{

        if i > row_index {
          let cmp = row[col_index];
          // println!("BCMP {} <= {}", val, cmp);
          if val <= cmp  {
            is_tallest_b = false;
            break;
          }
        }
        i += 1;
      }
      if is_tallest_b {
       // println!("BCOL val: {}, at: {},{}", val, row_index, col_index);
      }





  return is_tallest_a || is_tallest_b;
}

fn get_tree_score(data: &Vec<Vec<u32>>, row_index: usize, col_index: usize) -> i64 {

  let mut score : i64 = 0;
  let mut total : i64 = 1;
  let val = data[row_index][col_index];
  let debug = false;


  let (left, right) = data[row_index].split_at(col_index);

  if debug {
    println!("COMP SCORE val: {}, at: {},{}", val, row_index, col_index);
    println!("left: {:?} right: {:?}", left, right);
  }

  //LEFT
  let mut x = left.len();
  while x > 0 {
    x = x-1;
    let cell = left[x];
    if cell < val {
      score = score +1;
    } else if cell == val{
      score = score +1;
      break;
    } else {
      break;
    }
  }
  if score > 0 {
    if debug {
      println!("left score: {}", score);
    }
    total = total * score;
  }

  // RIGHT
  score = 0;
  for cell in right.iter().skip(1) {
    if cell < &val {
      score = score +1;
    } else if cell == &val {
      score = score +1;
      break;
    } else {
      break;
    }
  }
  if score > 0 {
    if debug {
      println!("right score: {}", score);
    }
    total = total * score;
  }


  //TOP
  score = 0;
  let mut y = row_index;
  while y > 0 {
    y = y -1;
    let cell = data[y][col_index];
    if cell < val {
      score = score +1;
    } else if cell == val {
      score = score +1;
      break;
    } else {
      break;
    }
  }
  if score > 0 {
    if debug {
      println!("top score: {}", score);
    }
    total = total * score;
  }

  score = 0;
  y = row_index + 1;
  while y < data.len() {
    let cell = data[y][col_index];
    y = y + 1;
    if cell < val {
      score = score +1;
    } else if cell >= val {
      score = score +1;
      break;
    }
  }
  if score > 0 {
    if debug {
      println!("bottom. score: {}", score);
    }
    total = total * score;
  }

  if debug {
    println!("total: {}", total);
    println!("-------===========------------");
  }
  return total;



}