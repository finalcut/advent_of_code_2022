use aoc_util::read_file;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
struct Coord {
    x: i16,
    y: i16,
}

// basically implementing a to_string method for the Coord struct
impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Debug, Clone)]
struct Knot {
    position: Coord,
}

#[derive(Debug, Clone)]
struct Rope {
    knots: Vec<Knot>,
}

fn main() {

  // passes withs sample data; but not real data.
  // real answer for my data was 6236 and it was correct. with this new implementation I get 5342 - so pretty far off.
  // try with values.txt
    let part1_moves = read_file("part_1_sample.txt").expect("Could not load values");
    do_it(part1_moves, 2, "Part 1:");

    // sample data should result in 36 but I get 28
    let part2_moves = read_file("part_2_sample.txt").expect("Could not load values");
    do_it(part2_moves, 10, "Part 2:");
}

fn do_it(moves: Vec<String>, rope_len: usize, caption: &str) {
    let mut rope = get_rope(rope_len);
    let mut results: Vec<String> = Vec::new();

    results.push(rope.knots[0].position.to_string()); // starting point

    let mut directions: HashMap<String, Coord> = HashMap::<String, Coord>::new();
    directions.insert("R".to_string(), Coord { x: 1, y: 0 });
    directions.insert("L".to_string(), Coord { x: -1, y: 0 });
    directions.insert("U".to_string(), Coord { x: 0, y: 1 });
    directions.insert("D".to_string(), Coord { x: 0, y: -1 });

    for line in moves {
        let move_info: Vec<&str> = line.split_whitespace().collect();
        let dir = move_info[0];
        let distance_to_travel: usize = move_info[1].parse().unwrap();

        for _i in 0..distance_to_travel {
            rope.knots[0].position.x += directions.get(dir).unwrap().x;
            rope.knots[0].position.y += directions.get(dir).unwrap().y;

            for x in 1..rope_len {
              move_tail(rope.knots[x - 1].position.clone(), &mut rope.knots[x].position);
            }
            results.push(rope.knots[rope_len - 1].position.to_string());
        }
    }

    results.sort_unstable(); // sort so we are sure we don't end up with repeating points.
    results.dedup(); // remove duplicate contiguous points



    println!("{}: {}", caption, results.len());
   // println!("RESULTS:  {:?}", results);
}

// takes in a mutable reference to the tail's coordinate position.
// updating it here persists back to the caller.
fn move_tail(head: Coord, tail: &mut Coord)  {

    let x_diff = head.x - tail.x;
    let y_diff = head.y - tail.y;

    if x_diff == 0 && y_diff == 0 {
        return;
    }

    if x_diff.abs() < 2 && y_diff.abs() < 2 {
        return;
    }

    if x_diff > 0 {
        tail.x += 1;
    } else if x_diff < 0 {
        tail.x -= 1;
    }

    if y_diff > 0 {
        tail.y += 1;
    } else if x_diff < 0 {
        tail.y -= 1;
    }

}

fn get_rope(size: usize) -> Rope {
    let mut rope = Rope { knots: [].to_vec() };

    for _i in 0..size {
        rope.knots.push(Knot {
            position: Coord { x: 0, y: 0 },
        });
    }
    return rope;
}
