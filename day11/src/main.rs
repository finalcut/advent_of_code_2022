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
      some info on modular arithmetic that helped me grok this as you read this
      bit of math theory.. think of the `factor` value as `m` | we will NEVER put a number as big as m in the
      "worry" array so our math will never get crazy big.  This is pretty freaking cool. Its sort of a simple
      way of encoding numbers so long as you don't care that many numbers encode to the same end result.

      So in the example 75 mod 17 = 7  it will end up being treated the same as 41 because 41 mod 17 = 7 as well.
      in terms of worry they are just degrees of the same "worry"

      in this example above both 75 and 41 are called congruent modulo 17 (same remainder when modulod against 17).  f
      see https://www.doc.ic.ac.uk/~mrh/330tutor/ch03.html

      In modular arithmetic, the set of numbers is limited so that only numbers
        0,1,2,...,m−1 are used, where m is a constant. Each number x is represented
        by the number x mod m: the remainder after dividing x by m. For example, if
        m = 17, then 75 is represented by 75 mod 17 = 7.
        Often we can take remainders before doing calculations. In particular, the
        following formulas hold:
        (x+ y) mod m = (x mod m+ y mod m) mod m
        (x− y) mod m = (x mod m− y mod m) mod m
        (x · y) mod m = (x mod m· y mod m) mod m
        x
        n mod m = (x mod m)
        n mod m


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