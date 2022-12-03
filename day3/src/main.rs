use array_tool::vec::Intersect;
use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn main() -> std::io::Result<()> {
    let file_name = "/values.txt";
    let path = env::current_dir()?;
    let input_file = path.display().to_string() + file_name;
    let rucks = lines_from_file(input_file).expect("Could not load values");

    // put a underscore at the beginning so I don't have to do index math of +1 all the time
    // position in the string is the priority of the letter
    let priority_order = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let mut priority_total = 0;

    let mut three_rucks = reset_three_rucks();

    let mut three_rucks_priority_total = 0;
    for ruck in rucks {
        // PART 1
        let mid = ruck.len() / 2;
        let (c1, c2) = ruck.split_at(mid);
        let item = common_char_in_strings(&vec![c1.into(), c2.into()]);

        let priority = priority_order.find(&item).unwrap();
        priority_total += priority;

        // PART 2
        if three_rucks[0].eq("") {
            three_rucks[0] = ruck.into();
        } else {
            if three_rucks[1].eq("") {
              three_rucks[1] = ruck.into();
            } else {
                // 3
                three_rucks[2] = ruck.into();
                let item = common_char_in_strings(&three_rucks);
                let priority = priority_order.find(&item).unwrap();
                three_rucks_priority_total += priority;
                three_rucks = reset_three_rucks();
                // println!("Round {}, Char: {}, Priority: {}, Total: {}", r / 3, item, priority, three_rucks_priority_total);
            }
        }
    }
    println!("Round 1 Result = {}", priority_total);
    println!("Round 2 Result = {}", three_rucks_priority_total);

    Ok(())
}

fn reset_three_rucks() -> Vec<String> {
  return vec!["".to_owned(), "".to_owned(), "".to_owned()];
}

fn common_char_in_strings(v: &[String]) -> String {
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

// function found at https://stackoverflow.com/a/35820003
fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}
