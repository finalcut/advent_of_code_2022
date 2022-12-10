use std::fmt;
use aoc_util::read_file;
use advent_of_code_ocr::parse_string_to_letters;
struct Signal {
    during: i64,
  //  after: i64,
}

struct Row {
    pixels: Vec<char>,
}

struct Grid {
    rows: Vec<Row>,
}

// basically implementing a to_string method for the Coord struct
impl fmt::Display for Grid {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut str : Vec<String> = Vec::new();
    for line in &self.rows {
      let l: String = line.pixels.iter().collect();
      str.push(l);
    }
    let result = str.join("\n");
    write!(f, "{}", result)
  }
}

fn main() {
    let data = read_file("values.txt").expect("Could not load values");

    let mut x_register_value: Vec<i64> = Vec::new(); // tracking register value at the end of each cycle
    let mut signal_strength: Vec<Signal> = Vec::new();

    let mut grid = get_empty_grid(40, 6);

    x_register_value.push(1);
    signal_strength.push(Signal {
        during: 0,
      //  after: 0,
    }); // lets me not think about 0 indexed stuff..

    let mut cycle: usize = 0;
    for line in data {
        //println!("LINE: {:?}", line);
        cycle += 1;
        let instruction: Vec<&str> = line.split_whitespace().collect();
        let command = instruction[0];

        // noop and addr both dont change the the counter yet.
        // let x = x_register_value.last().unwrap().clone();
        let x: i64 = x_register_value.last().unwrap().clone();
        x_register_value.push(x);
        let s1 = compute_strength(x, cycle);

        signal_strength.push(Signal {
            during: s1,
            // after: s1,
        });

        if command.eq("noop") {
            continue;
        }

        // command eq addx process
        cycle += 1;

        if instruction.len() > 1 {
            let rx: i64 = x_register_value.last().unwrap().clone();
            let adjustment: i64 = instruction[1].parse().unwrap();
            let r2 = rx + adjustment;

            let sig = Signal {
                during: compute_strength(rx, cycle),
                // after: compute_strength(r2, cycle),
            };

            signal_strength.push(sig);
            x_register_value.push(r2);
        }
    }

    let part1 = sum_cycles(&signal_strength, [20, 60, 100, 140, 180, 220].to_vec());
    println!("part1: {:?}", part1);

    // part 2
    println!("part2:");
    light_up_grid(&mut grid, &x_register_value);
    draw_grid(&grid);
}

fn compute_strength(register: i64, cycle: usize) -> i64 {
    return register * cycle as i64;
}

fn sum_cycles(signals: &Vec<Signal>, points: Vec<usize>) -> i64 {
    let mut sum = 0;
    for point in points {
        sum += signals[point].during;
    }
    return sum;
}

fn get_empty_grid(width: i16, height: i16) -> Grid {
    let mut grid = Grid { rows: Vec::new() };

    for _i in 0..height {
        let mut row = Row { pixels: Vec::new() };
        for _x in 0..width {
            /*
             instructions said to use a . for this; but it makes the final screen hard to read
             so I used a {space} and the rendering is MUCH easier to read
            */
            row.pixels.push('.');
        }
        grid.rows.push(row);
    }
    return grid;
}

fn draw_grid(grid: &Grid) {

  for row in &grid.rows {
        let pix = row.pixels.clone();
        let _line: String = pix.into_iter().collect();
        println!("{:?}", _line);
    }
    println!("== {:?}", parse_string_to_letters(&grid.to_string()));
}

fn light_up_grid(grid: &mut Grid, x_values: &Vec<i64>) {
    let width = grid.rows[0].pixels.len();


    for g in 0..grid.rows.len() {
        let start = g * width;
        let end = start + width;

        let mut point: i64 = 0;
        for cycle in start..end {
            /*
                  we always want the register value DURING the cycle not at the end
                  since we start with a zero based index this works perfectly relative
                  to how I stored register values at each index for the end of that cycle.
                  so in x_values[1] it has the value for the end of cycle 1.. the register value
                  during cycle 1 wwould be at x_values[0]
                  i got kind of lucky there.
            */
            let x = x_values[cycle];
            let sprite_pos = [x - 1, x + 1];

            if point >= sprite_pos[0] && point <= sprite_pos[1] {
                grid.rows[g].pixels[point as usize] = '#';
            }

            point += 1;
        }
    }
}
