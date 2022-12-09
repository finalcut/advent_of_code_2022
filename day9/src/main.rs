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
    let moves = read_file("values.txt").expect("Could not load values");

    let mut directions: HashMap<String, Coord> = HashMap::<String, Coord>::new();
    directions.insert("U".to_string(), Coord { x: 0, y: 1 });
    directions.insert("D".to_string(), Coord { x: 0, y: -1 });
    directions.insert("L".to_string(), Coord { x: -1, y: 0 });
    directions.insert("R".to_string(), Coord { x: 1, y: 0 });

    let mut rope = get_rope(2);

    let mut results: Vec<String> = Vec::new();

    for line in moves {
        let move_info: Vec<&str> = line.split_whitespace().collect();
        let dir = move_info[0];
        let dist: i16 = move_info[1].parse().unwrap();

        let tail_pos = rope.knots.len() - 1;

        for _i in 0..dist {
            rope.knots[0].position.x += directions.get(dir).unwrap().x;
            rope.knots[0].position.y += directions.get(dir).unwrap().y;

            for x in 0..tail_pos {
                rope.knots[tail_pos].position = calculate(&rope.knots[x].position, &rope.knots[x + 1].position);
            }

            results.push(rope.knots[tail_pos].position.to_string());
        }
    }

    results.sort_unstable();
    results.dedup();
    println!("{:?}", rope);

    println!("part1: {}", results.len());
}

fn calculate(head: &Coord, tail: &Coord) -> Coord {
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
    let mut rope = Rope {
        knots: [].to_vec(),
    };

    for _i in 0..size {
      rope.knots.push(Knot {position: Coord { x: 0, y: 0 }});
    }
    return rope;
}
