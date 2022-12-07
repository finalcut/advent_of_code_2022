use aoc_util::{get_test_data};
use dict::{Dict, DictIface};

/*


ElfFile {
  Name:
  Size:

}
DirData {
  name: String
  dirs: Vec<DirData>
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
struct FileData {
    size: i64,
    name: String
}
#[derive(Debug, Clone)]
struct DirData {
    name: String,
    files: Vec<FileData>,
}

fn main() {
  let input = get_test_data().expect("Could not load values");


  let mut current_node = create_elf_dir("/");
  //let mut parent_dir = &mut current_node;
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
      current_node.files.push(create_elf_file(parts[1], parts[0]));

    }

    if line[0..4].eq(DIR){
      let name = &line[4..(line.len())];
      //current_node.dirs.push(create_elf_dir(&name));
    }

    // changing directory.
    if line[0..5].eq(CD){
      let parts : Vec<&str> = line.split_whitespace().collect();
      let name = parts[2].trim();
      println!("name : {:#?}", name);

      if !name.eq("..") {
        // find the directory in the current_node.dirs with the name part of this line

       //let idx = current_node.dirs.iter().position(|r| r.name.eq(&name)).unwrap();
        //println!("CURRENT : {:#?}", idx);
        //parent_dir = &mut current_node;
        //current_node = &mut current_node.dirs[idx];
        depth = depth +1;
      } else {
        depth = depth -1;
        //current_node = &mut parent_dir;
      }

    }

  }
  println!("CURRENT : {:#?}", current_node);


}


fn create_elf_dir(name: &str) -> DirData {
  DirData {
    name: name.to_string(),
    files : Vec::new(),
  }
}

fn create_elf_file(name: &str, size: &str) -> FileData {
  FileData {
    name: name.to_string(),
    size: size.parse::<i64>().unwrap()
  }
}
