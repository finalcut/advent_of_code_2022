use aoc_util::{get_seed_data};

fn main() {
  let input = get_seed_data().expect("Could not load values");

  let mut consumed_count = 0;
  let mut overlapped_count = 0;
  for i in input {
    let parts : Vec<&str> = i.split(",").collect();

    let sect1 : Vec<&str> = parts[0].split("-").collect();
    let s1: Vec<u32> = sect1.iter().flat_map(|x| x.parse()).collect();

    let sect2 : Vec<&str> = parts[1].split("-").collect();
    // converts each of the sub vectors into numbers from strings and removes any values that can't become numbers (blanks)
    let s2: Vec<u32> = sect2.iter().flat_map(|x| x.parse()).collect();


    if section_fully_includes_section(&s1,&s2){
      consumed_count += 1;
    } else if section_fully_includes_section(&s2,&s1){
      consumed_count += 1;
    }

    if section_overlaps_section(&s1,&s2){
      overlapped_count += 1;
    } else if section_overlaps_section(&s2,&s1){
      overlapped_count += 1;
    }

  }

  println!("part1: {}", consumed_count);

  println!("part2: {}", overlapped_count);

}

fn section_fully_includes_section(s1: &Vec<u32>, s2: &Vec<u32>) -> bool {
  return s1[0] <= s2[0] && s1[1] >= s2[1];
}

fn section_overlaps_section(s1: &Vec<u32>, s2: &Vec<u32>) -> bool {
  return s2[0] <= s1[1] && s2[1] >= s1[1];
}
