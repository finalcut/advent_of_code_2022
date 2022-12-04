use std::{
  env,
  fs::File,
  io::{self, BufRead, BufReader},
  path::Path,
};
use array_tool::vec::Intersect;



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