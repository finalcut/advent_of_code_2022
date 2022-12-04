use aoc_util::{get_seed_data,};

fn main() {
  let input = get_seed_data().expect("Could not load values");

  let mut consumed_count = 0;
  let mut overlapped_count = 0;
  for i in input {

    // standardize my delimiter
    let x = str::replace(&i, "-", ",");

    //convert to vector of numbers
    let section_borders: Vec<i32> = x
        .split(",")
        .map(|s| s.parse().expect("parse error"))
        .collect();

    if section_fully_includes_section(&section_borders){
      consumed_count += 1;
    } else if section_fully_includes_section(&section_borders){
      consumed_count += 1;
    }

    if section_overlaps_section(&section_borders){
      overlapped_count += 1;
    } else if section_overlaps_section(&section_borders){
      overlapped_count += 1;
    }
  }

  println!("part1: {}", consumed_count);

  println!("part2: {}", overlapped_count);

}

fn section_fully_includes_section(section_borders: &Vec<i32>) -> bool {
  return section_borders[0] <= section_borders[2] && section_borders[1] >=section_borders[3];
}

fn section_overlaps_section(section_borders: &Vec<i32>) -> bool {
  return section_borders[2] <= section_borders[1] && section_borders[3] >= section_borders[0];
}
