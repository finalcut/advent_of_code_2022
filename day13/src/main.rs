/*
  Notes:
    numbers as single character strings such as "6" and "5" have standard comparison operations like < and > and act just like their numeric counterparts


*/
fn main() {

  let x = "[1,[2,[3,[4,[5,6,7]]]],8,9]";
  let y = "[1,[2,[3,[4,[5,6,0]]]],8,9]";

  let jx = json::parse(x).unwrap();
  let jy = json::parse(y).unwrap();

  let mut correct = false;

  println!("{:?}",jx[0].is_number());
  if jx.len() != jy.len() {
    for i in 0..jx.len() {
      let left = jx[i].clone();
      let right= jy[i].clone();

      if left.is_number() && right.is_number() {
        if left.as_i16().unwrap() > right.as_i16().unwrap() {
          break; // bigger number on right than on left.. not ordered right.
        }
      }

      if left.is_number() && right.is_array() {
        let left = json::array![left.as_i16().unwrap()];

      }
    }
 }

}

fn is_list_less_than_list(x : json::JsonValue, y: json::JsonValue) -> bool {

  return false;
}