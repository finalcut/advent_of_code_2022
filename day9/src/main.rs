use aoc_util::read_file;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
struct Coord {
    x: i16,
    y: i16,
}

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

    let ti = rope_len - 1;
    results.push(rope.knots[ti].position.to_string()); // starting point

    let mut directions: HashMap<String, Coord> = HashMap::<String, Coord>::new();
    directions.insert("R".to_string(), Coord { x: 1, y: 0 });
    directions.insert("L".to_string(), Coord { x: -1, y: 0 });
    directions.insert("U".to_string(), Coord { x: 0, y: 1 });
    directions.insert("D".to_string(), Coord { x: 0, y: -1 });

    for line in moves {
        let move_info: Vec<&str> = line.split_whitespace().collect();
        let dir = move_info[0];
        let dist: i16 = move_info[1].parse().unwrap();

        for _i in 0..dist {
            rope.knots[0].position.x += directions.get(dir).unwrap().x;
            rope.knots[0].position.y += directions.get(dir).unwrap().y;

            for x in 1..rope_len {
                rope.knots[x].position =
                    get_new_tail(&rope.knots[x - 1].position, &rope.knots[x].position);
            }
            results.push(rope.knots[ti].position.to_string());
        }
    }

    results.sort_unstable(); // sort so we are sure we don't end up with repeating points.
    results.dedup(); // remove duplicate contiguous points



    println!("{}: {}", caption, results.len());
   // println!("RESULTS:  {:?}", results);
}

fn get_new_tail(head: &Coord, tail: &Coord) -> Coord {
    let mut t = tail.clone();

    let x_diff = head.x - tail.x;
    let y_diff = head.y - tail.y;

    if x_diff == 0 && y_diff == 0 {
        return tail.clone();
    }

    if x_diff.abs() < 2 && y_diff.abs() < 2 {
        return tail.clone();
    }

    if x_diff > 0 {
        t.x += 1;
    } else if x_diff < 0 {
        t.x -= 1;
    }

    if y_diff > 0 {
        t.y += 1;
    } else if x_diff < 0 {
        t.y -= 1;
    }

    return t.clone();
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
