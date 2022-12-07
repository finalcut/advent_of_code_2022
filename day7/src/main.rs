use aoc_util::{get_test_data};


/*


ElfFile {
  Name:
  Size:

}
ElfDir {
  name: String
  dirs: Vec<ElfDir>
  files: Vec<ElfFile>
}


*/

const DIR: &str = "dir ";
const CD: &str  = "$ cd ";
const LS: &str  = "$ ls";

enum ReadMode {
  LS,
  CD,
}

#[derive(Debug, Clone)]
struct ElfFile {
    size: i64,
    name: String
}
#[derive(Debug, Clone)]
struct ElfDir {
    name: String,
    dirs: Vec<ElfDir>,
    files: Vec<ElfFile>,
}

fn main() {
  let input = get_test_data().expect("Could not load values");

  let mut tree = create_elf_dir("/");
  let mut current_dir = &mut tree;
  //let mut parent_dir = &mut current_dir;
  let mut depth : i16 = 0;

  let mut mode : ReadMode = ReadMode::CD;
  for line in input.iter().skip(1) {
    if line.eq("$ ls"){
      mode = ReadMode::LS;
      continue;
    }


    // FOUND A FILE; put it in the current DIR
    if line.chars().next().unwrap().is_digit(10) {
      let parts : Vec<&str> = line.split_whitespace().collect();
      current_dir.files.push(create_elf_file(parts[1], parts[0]));

    }

    if line[0..4].eq(DIR){
      let name = &line[4..(line.len())];
      current_dir.dirs.push(create_elf_dir(&name));
    }

    // changing directory.
    if line[0..5].eq(CD){
      let parts : Vec<&str> = line.split_whitespace().collect();
      let name = parts[2].trim();
      println!("name : {:#?}", name);

      if !name.eq("..") {
        // find the directory in the current_dir.dirs with the name part of this line

       let idx = current_dir.dirs.iter().position(|r| r.name.eq(&name)).unwrap();
        println!("CURRENT : {:#?}", current_dir);
        //println!("CURRENT : {:#?}", idx);
        //parent_dir = &mut current_dir;
        current_dir = &mut current_dir.dirs[idx];
        depth = depth +1;
      } else {
        depth = depth -1;
        //current_dir = &mut parent_dir;
      }

    }

  }


}


fn create_elf_dir(name: &str) -> ElfDir {
  ElfDir {
    name: name.to_string(),
    dirs : Vec::new(),
    files : Vec::new(),
  }
}

fn create_elf_file(name: &str, size: &str) -> ElfFile {
  ElfFile {
    name: name.to_string(),
    size: size.parse::<i64>().unwrap()
  }
}
