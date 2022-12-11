/*
  this is basically a copy/paste of someone elses solution because I did even remember MOD math existed and my efforts with libraries for handling bigger values than i128
  was going to take WAY too long to run.  I'm pretty sure I had it right but I'll never know.

  source of solution: https://www.reddit.com/r/adventofcode/comments/zifqmh/2022_day_11_solutions/izsr2lq/
  that reddit comment has a link to a site where their code was posted.  The link is huge so I'm not including it.
*/

use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Clone, Debug)]
enum Op {
    #[display(r"old * old")]
    Square,
    #[display(r"old + {0}")]
    Add(i64),
    #[display(r"old * {0}")]
    Multiply(i64),
}

#[derive(Debug, Clone, FromStr, Display)]
#[display(r"Monkey {id}:|  Starting items: {worry_string}|  Operation: new = {operation}|  Test: divisible by {divisor}|    If true: throw to monkey {pass_monkey}|    If false: throw to monkey {fail_monkey}")]
struct Monkey {
    id: usize,
    operation: Op,
    divisor: i64,
    worry_string: String,
    pass_monkey: usize,
    fail_monkey: usize,
    #[from_str(ignore, default)]
    worry: Vec<i64>,
    #[from_str(ignore, default)]
    inspected: usize,
}


fn main() {
//    let observations: Vec<String> = read_file("test.txt").expect("Could not load values");


    let input: Vec<Monkey> = include_str!("../values.txt")
    .split("\n\n")
    .map(|block| block.trim())
    .map(|block| {
        let block = block.replace("\n", "|");
        let mut monkey: Monkey = block.parse().unwrap();
        let items = monkey
            .worry_string
            .split(", ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        monkey.worry = items;
        monkey
    })
    .collect();

    let mut monkeys = input.clone();
    /*
     this link help explains why this factor works: https://www.reddit.com/r/adventofcode/comments/ziw4aq/2022_day_11_part_2_so_about_the_trick/izsr5av/
     basically, the product of all the divisors is the smallest common multiple. I still don't fully understand it.. but I'm doing some reading.
    */
    let factor: i64 = monkeys.iter().map(|m| m.divisor).product();

    for _ in 0..10000 {
      for i in 0..monkeys.len() {
          let m = monkeys[i].clone();
          for old in m.worry {
              let new = match m.operation {
                  Op::Square => old * old,
                  Op::Add(x) => old + x,
                  Op::Multiply(x) => old * x,
              }  % factor; // for part 1 - put a div/3 after new is computed before modding by factor.
              if new % m.divisor == 0 {
                  monkeys[m.pass_monkey].worry.push(new);
              } else {
                  monkeys[m.fail_monkey].worry.push(new);
              }
          }
          monkeys[i].inspected += monkeys[i].worry.len();
          monkeys[i].worry.clear();
      }
  }
  let monkey_business: usize = monkeys
      .iter()
      .map(|m| m.inspected)
      .sorted()
      .rev()
      .take(2)
      .product();
  println!("{monkey_business}");



  }