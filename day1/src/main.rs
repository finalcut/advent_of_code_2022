use std::{
  fs::File,
  io::{self, BufRead, BufReader},
  path::Path,
};



fn main() {

  let input = lines_from_file("/Users/Bill.Rawlinson@mheducation.com/dev/aoc/day1/src/values.txt").expect("Could not load values");

  let output = split_delimited(&input, &"".to_owned());

  let mut sum_vec : Vec<u32> = Vec::with_capacity(output.len());

  for &i in &output {
    let number_vec: Vec<u32> = i.iter().flat_map(|x| x.parse()).collect(); // converts each of the sub vectors into numbers from strings and removes any values that can't become numbers (blanks)
    let sum : u32 = number_vec.iter().sum();
    sum_vec.push(sum);
  }

    sum_vec.sort();
    sum_vec.reverse();

    println!("Richest Elf: {:?}", sum_vec[0]);

    sum_vec.truncate(3);
    let top3 : u32 = sum_vec.iter().sum();
    println!("Top Three Elves: {:?}", top3);
}


pub fn split_delimited<'a, T>(input: &'a [T], delim: &T) -> Vec<&'a [T]>
where
    T: PartialEq<T>,
{
    let elems = input.iter().enumerate();
    let (k, mut r) = elems.fold((0, vec![]), |(i, mut r), (j, x)| {
        if x == delim && j > 0 {
            r.push(&input[i..j]);
            return (j, r);
        }
        (i, r)
    });
    if !input.is_empty() {
        r.push(&input[k..]);
    }
    r
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
  BufReader::new(File::open(filename)?).lines().collect()
}
