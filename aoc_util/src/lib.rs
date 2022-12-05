use std::{
  env,
  fs::File,
  io::{self, BufRead, BufReader},
  path::Path,
};
use array_tool::vec::Intersect;
use lazy_static::lazy_static;
use regex::Regex;


// function found at https://stackoverflow.com/a/35820003
fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
BufReader::new(File::open(filename)?).lines().collect()
}

pub fn get_seed_data() -> io::Result<Vec<String>> {
let file_name = "/values.txt";
let path = env::current_dir()?;
let input_file = path.display().to_string() + file_name;
return lines_from_file(input_file);
}

pub fn common_char_in_strings(v: &[String]) -> String {
  let mut result: Vec<char> = v[0].chars().collect();

  for s in v {
      let vec : Vec<char> = s.chars().collect();
      result = result.intersect(vec);
  }

  return if result.len() > 0 {
    result[0].to_string()
  } else {
      "".to_string()
  };
}

// function found at https://www.reddit.com/r/rust/comments/hgcpds/how_to_split_a_vector_by_an_entry_and_collect_all/fw5c5ml/
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



pub fn str_strip_numbers(s: &str) -> Vec<i64> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+").unwrap();
    }
    // iterate over all matches
    RE.find_iter(s)
        // try to parse the string matches as i64 (inferred from fn type signature)
        // and filter out the matches that can't be parsed (e.g. if there are too many digits to store in an i64).
        .filter_map(|digits| digits.as_str().parse().ok())
        // collect the results in to a Vec<i64> (inferred from fn type signature)
        .collect()
}