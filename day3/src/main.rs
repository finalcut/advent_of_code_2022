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
    let keys = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let mut priority_total = 0;

    let mut first_ruck = "".to_owned();
    let mut second_ruck = "".to_owned();
    let mut group_ruck_count = 0;
    let mut group_priority_total = 0;
    for ruck in rucks {
        // PART 1
        let mid = ruck.len() / 2;
        let (c1, c2) = ruck.split_at(mid);
        let item = common_char_in_strings(&vec![c1.into(), c2.into()]);

        let priority = keys.find(&item).unwrap();
        priority_total += priority;

        // PART 2
        if first_ruck.eq("") {
            group_ruck_count = 1;
            first_ruck = ruck;
        } else {
            group_ruck_count += 1;
            if group_ruck_count == 2 {
                second_ruck = ruck;
            } else {
                // 3
                let item = common_char_in_strings(&vec![first_ruck.into(), second_ruck.into(), ruck.into()]);
                let priority = keys.find(&item).unwrap();
                group_priority_total += priority;
                first_ruck = "".to_owned();
                second_ruck = "".to_owned();
                // println!("Round {}, Char: {}, Priority: {}, Total: {}", r / 3, item, priority, group_priority_total);
            }
        }
    }
    println!("Round 1 Result = {}", priority_total);
    println!("Round 2 Result = {}", group_priority_total);

    Ok(())
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
