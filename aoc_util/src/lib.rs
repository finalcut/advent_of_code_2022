use std::{
  env,
  fs::File,
  io::{self, BufRead, BufReader},
  path::Path,
};



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
