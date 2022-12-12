


const LOOPER : i16 = 12;
// NOTE: if you set looper >= 13 method 2 and method 3 will generate a 'attempt to multiply with overflow' error.  The numbers are too big!

/*
  OUTPUT when LOOPER = 12

  method 1
  ------------------
  Monkey 1: [96, 96, 9, 37, 23, 93, 52, 27, 48, 48, 14, 14, 63]
  Monkey 2: [50, 60, 0]
  Monkey 3: []

  method 2
  ------------------
  Monkey 1: [1806690235446, 1806690235446, 8426128726971174, 21473037723571702, 14949583225271438, 292196399403032658, 42480114410725162, 480996554157342, 26715098778077058, 956307364785864678, 491344577958584, 6996728565183224, 1646861620099057488]
  Monkey 2: [23457574783280, 2443810288694686020, 18866474063469480]
  Monkey 3: []

  method 3
  ------------------
  Monkey 1: [1806690235446, 1806690235446, 8426128726971174, 21473037723571702, 14949583225271438, 292196399403032658, 42480114410725162, 480996554157342, 26715098778077058, 956307364785864678, 491344577958584, 6996728565183224, 1646861620099057488]
  Monkey 2: [23457574783280, 2443810288694686020, 18866474063469480]
  Monkey 3: []

*/

fn main() {

  let a: Vec<i64> = [93, 54, 69, 66, 71].to_vec();
  let b: Vec<i64> = [89, 51, 80, 66].to_vec();
  let c: Vec<i64> = [90, 92, 63, 91, 96, 63, 64].to_vec();

  method1(a.clone(),b.clone(),c.clone());
  method2(a.clone(),b.clone(),c.clone());
  method3(a.clone(),b.clone(),c.clone());

}

// fast way that doesnt generate huge numbers and uses a factor..
fn method1(mut m0 : Vec<i64>, mut m1: Vec<i64>, mut m2: Vec<i64>){

      /*


      some info on modular arithmetic that helped me grok this as you read this
      bit of math theory..

      First, read this reddit comment.. It really helps explain why the product of all the divisors is the right thing to work with:
      https://www.reddit.com/r/adventofcode/comments/zizi43/2022_day_11_part_2_learning_that_it_was_that/iztt8mx/


      think of the `factor` value as `m` | we will NEVER put a number as big as m in the
      "worry" array so our math will never get crazy big.  This is pretty freaking cool. Its sort of a simple
      way of encoding numbers so long as you don't care that many numbers encode to the same end result.

      So in the example 75 mod 17 = 7  it will end up being treated the same as 41 because 41 mod 17 = 7 as well.
      in terms of worry (see puzzle for defn of worry) they are just degrees of the same "worry"

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


  let factor = 105;
  let mut  modus = 3;

  for _x in 0..LOOPER {
    for i in 0..m0.len() {
      //  Operation: new = old * 19
      //  If true: throw to monkey 2
      // If false: throw to monkey 1

      let new = (m0[i] * 19) % factor;
      if new % modus == 0 {
        m2.push(new);
      } else {
        m1.push(new);
      }
    }
    m0.clear();

    modus = 7;
    for i in 0..m1.len() {
      //   new = old + 6
      //      If true: throw to monkey 2
      // If false: throw to monkey 0


      let new = (m1[i] + 6) % factor;
      if new % modus == 0 {
        m2.push(new);
      } else {
        m0.push(new);
      }
    }
    m1.clear();

    modus = 5;
    for i in 0..m2.len() {
    /*
        Operation: new = old * old
        Test: divisible by 5
          If true: throw to monkey 1
          If false: throw to monkey 0

    */
      let new = (m2[i] + m2[i]) % factor;
      if new % modus == 0 {
        m1.push(new);
      } else {
        m0.push(new);
      }
    }
    m2.clear();
  }

  print_results(m0, m1, m2, 1);


}

// my way, whcih generates huge numbers eventually.. dont use the factor.
fn method2(mut m0 : Vec<i64>, mut m1: Vec<i64>, mut m2: Vec<i64>){
  let mut  modus = 3;

  for _x in 0..LOOPER {
    for i in 0..m0.len() {
      //  Operation: new = old * 19
      //  If true: throw to monkey 2
      // If false: throw to monkey 1

      let new = m0[i] * 19;
      if new % modus == 0 {
        m2.push(new);
      } else {
        m1.push(new);
      }
    }
    m0.clear();

    modus = 7;
    for i in 0..m1.len() {
      //   new = old + 6
      //      If true: throw to monkey 2
      // If false: throw to monkey 0


      let new = m1[i] + 6;
      if new % modus == 0 {
        m2.push(new);
      } else {
        m0.push(new);
      }
    }
    m1.clear();

    modus = 5;
    for i in 0..m2.len() {
    /*
        Operation: new = old * old
        Test: divisible by 5
          If true: throw to monkey 1
          If false: throw to monkey 0

    */
      let new = m2[i] + m2[i];
      if new % modus == 0 {
        m1.push(new);
      } else {
        m0.push(new);
      }
    }
    m2.clear();
  }

  print_results(m0, m1, m2, 2);

}


/*
  trying to use this info:
  (k + (x mod n)) mod n == (x + k) mod n
  (k * (x mod n)) mod n == (x * k) mod n
  (x mod (m * n)) mod n ==  x      mod n
*/
fn method3(mut m0 : Vec<i64>, mut m1: Vec<i64>, mut m2: Vec<i64>){

  let mut  modus = 3;

  for _x in 0..LOOPER {
    for i in 0..m0.len() {
      //  Operation: new = old * 19
      //  If true: throw to monkey 2
      // If false: throw to monkey 1

      let new = m0[i] * 19;
      if (19 * (m0[i] % modus)) % modus == 0 {
        m2.push(new);
      } else {
        m1.push(new);
      }
    }
    m0.clear();

    modus = 7;
    for i in 0..m1.len() {
      //   new = old + 6
      //      If true: throw to monkey 2
      // If false: throw to monkey 0


      let new = m1[i] + 6;
      if (6 + (m1[i] % modus)) % modus == 0 {
        m2.push(new);
      } else {
        m0.push(new);
      }
    }
    m1.clear();

    modus = 5;
    for i in 0..m2.len() {
    /*
        Operation: new = old * old
        Test: divisible by 5
          If true: throw to monkey 1
          If false: throw to monkey 0

    */
      let new = m2[i] + m2[i];
      if (m2[i] + (m2[i] % modus)) % modus == 0 {
        m1.push(new);
      } else {
        m0.push(new);
      }
    }
    m2.clear();
  }

  print_results(m0, m1, m2, 3);

}

fn print_results(m0: Vec<i64>, m1: Vec<i64>, m2: Vec<i64>, method: usize) {
  println!("method {}", method);
  println!("------------------");
  println!("Monkey 1: {:?}", m0);
  println!("Monkey 2: {:?}", m1);
  println!("Monkey 3: {:?}", m2);
  println!();
}