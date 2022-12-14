use std::cmp::Ordering;
fn main() {
    let input: Vec<Vec<&str>> = include_str!("../values.txt")
        .split("\n\n")
        .map(|block| block.trim())
        .map(|block| { block.split("\n").collect() })
        .collect();

    let mut part1_good_indexes: Vec<usize> = [].to_vec();
    for c in 0..input.len() {
        let left = json::parse(input[c][0]).unwrap();
        let right = json::parse(input[c][1]).unwrap();

        // left is less than right so it is correct
        if compare(left, right) == -1  {
            part1_good_indexes.push(c + 1);
        }
    }

    // part 2..
    let mut flat: Vec<&str> = input.concat();
    // inject the dividers..
    flat.push("[[2]]");
    flat.push("[[6]]");

    flat.sort_by(|a, b| {
        let c = compare(json::parse(a).unwrap(), json::parse(b).unwrap());
        if c == 0 {
            Ordering::Equal
        } else if c == 1 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    // find the dividers indexes (+1)
    let d1 =
        flat
            .iter()
            .position(|r| r.eq(&"[[2]]"))
            .unwrap() + 1;
    let d2 =
        flat
            .iter()
            .position(|r| r.eq(&"[[6]]"))
            .unwrap() + 1;
    let decoder = d1 * d2;

    let sum: usize = part1_good_indexes.iter().sum();
    println!("part 1: {:?}", sum);
    println!("part 2: {:?} : {}, {}", decoder, d1, d2);
}

fn compare(left: json::JsonValue, right: json::JsonValue) -> i16 {
    print_out("left/right", left.clone(), right.clone());

    if left.is_array() && right.is_array() {
        let mut max = left.len();
        if right.len() > max {
            max = right.len();
        }
        for i in 0..max {
            if i >= left.len() {
                return -1;
            }
            if i >= right.len() {
                return 1;
            }

            let result = compare(left[i].clone(), right[i].clone());
            if result != 0 {
                return result;
            }
        }

        return 0; // they are the same..
    } else if left.is_number() & right.is_number() {
        if left.as_i16().unwrap() > right.as_i16().unwrap() {
            return 1;
        }
        if right.as_i16().unwrap() > left.as_i16().unwrap() {
            return -1;
        }
        return 0; // they are the same
    } else {
        if left.is_array() {
            return compare(left, json::array![right.as_i16().unwrap()]);
        } else {
            return compare(json::array![left.as_i16().unwrap()], right);
        }
    }
}

fn print_out(m: &str, a: json::JsonValue, b: json::JsonValue) {
    println!("msg: {}", m);
    println!("{} {:?}", a.len(), json::stringify(a.clone()));
    println!("{} {:?}", b.len(), json::stringify(b.clone()));
    println!();
}