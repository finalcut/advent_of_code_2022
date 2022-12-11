#[derive(Debug)]
enum Cmd {
  Noop,
  Addx(i32),
}

fn alternate() {
  let input: Vec<Cmd> = include_str!("../input")
      .lines()
      .map(|line| match line {
          "noop" => Cmd::Noop,
          _ => Cmd::Addx(line[5..].parse::<i32>().unwrap()),
      })
      .collect();

  part1(&input);
  part2(&input);
}

fn part1(input: &Vec<Cmd>) {
  let mut x: i32 = 1;
  let mut cycle: i32 = 1;
  let mut total: i32 = 0;

  for cmd in input {
      total += signal_strength(x, cycle);
      match cmd {
          Cmd::Noop => (),
          Cmd::Addx(operand) => {
              cycle += 1;
              total += signal_strength(x, cycle);
              x += operand;
          }
      }
      cycle += 1;
  }
  println! {"Part 1: {total}"};
}

fn part2(input: &Vec<Cmd>) {
  let mut x: i32 = 1;
  let mut cycle: i32 = 1;

  for cmd in input {
      pixel_out(x, cycle);
      match cmd {
          Cmd::Noop => (),
          Cmd::Addx(value) => {
              cycle += 1;
              pixel_out(x, cycle);
              x += value;
          }
      }
      cycle += 1;
  }
}

fn signal_strength(x: i32, cycle: i32) -> i32 {
  if (cycle - 20) % 40 == 0 {
      x * cycle
  } else {
      0
  }
}

fn pixel_out(x: i32, cycle: i32) {
  let pos = (cycle - 1) % 40;
  let pixel = if (x - pos).abs() <= 1 { "#" } else { "." };
  print!("{pixel}");
  if pos == 39 {
      println!()
  }
}