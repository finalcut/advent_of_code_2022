// Totally stolen from : https://github.com/SvetlinZarev/advent-of-code/blob/main/2022/aoc-day-12/src/lib.rs
// I had no idea how to do this.  I feel like I've forgotten a lot since college.

use std::collections::{VecDeque};

const DIR: [(isize, isize); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)]; // directions to move left, right, up, down

fn main() {

  let data = include_str!("../test.txt");
  let u8_grid = parse_u8_grid(data);

  let grid = get_grid(u8_grid.clone(),  b'S');
  println!("Part 1: {}", bfs_from_start_element(grid.clone()));

  let grid = get_grid(u8_grid.clone(),  b'a');
  println!("Part 2: {}", bfs_from_start_element(grid.clone()));

}

fn get_grid(grid: Vec<Vec<u8>>, target_symbol: u8) -> Grid {
  Grid {
    points : grid,
    target_symbol : target_symbol,
    start_symbol : b'E' // the instructions identify the E as the "end" but we actually start there and work backwards to our designated "start"
  }
}

pub fn parse_u8_grid<I: AsRef<str>>(input: I) -> Vec<Vec<u8>> {
  input
      .as_ref()
      .lines()
      .filter(|l| !l.is_empty())
      .map(|l| l.as_bytes().to_vec())
      .collect()
}



fn bfs_from_start_element(grid: Grid) -> u32 {
  let (mut start_row, mut start_col) = (0, 0);

  // the puzzle identified the E as the end.. but, really.. we want to start there and work backwards from there.
  for row in 0..grid.points.len() {
      for col in 0..grid.points[row].len() {
          if grid.points[row][col] == grid.start_symbol {
              (start_row, start_col) = (row, col);
              break;
          }
      }
  }

  bfs(grid, start_row, start_col).unwrap()
}

// this starts at the end point and works its way back to the start point..

/* explaination how BFS works..
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi

0. seed the queue with your starting points value (in this case, ascii z) and mark the starting point visited
1. pop the first item of your queue that's your current point
2. at your current point evaluate your neighbors to the left, right, above, below
3. for each neighbor that fits the rule (one letter smaller or equal in this case; by ascii code) put that neighbor on the back of the queue.
4. if the starting point isn't found.. repeat for each item that was added to the queue since you were last at step 1.
5. your queue now just has the "neighbors" of the points you've visited in this loop.
4. go back to part step one.

illustration:
0. [E] and step counter is 0.
1. [E].pop() => E (treat E as a z)
2. look at z,x,x,v which surround E.  only z matches rule of 1 or less difference
3. [].pus(z) => [z]; mark space with E visited; set to 0
4. we didn't see the target.  increment step counter
5. loop over queue again (effectively repeating steps 1-4 with current queue until target is found)
...
1. [z].pop() => z
2  look at s,y,u,0.  Only y meets; set space we're in to 0 to mark visited
3. [].push(y) => [y]
4. we didn't see target increment step counter
... repeat 1-4 with queue again..
*/

fn bfs(mut grid: Grid, start_row: usize, start_col: usize) -> Option<u32> {

  let mut queue = VecDeque::new();
  let mut steps : u32 = 0;

  // change the start to a zero, we've visited it.
  grid.points[start_row][start_col] = 0;
  // putting the tallest hill z in the queue. we treat the beginning element as a z (tallest hill) at spot found in bfs_from_start_element
  // remember, we're working backwards.
  queue.push_back((b'z', start_row, start_col));

  while !queue.is_empty() {

    //println!("grid: {:?}", grid.points);
    // not only do we need to go from 0 to queue.len so that we can avoid ugly borrowing stuff in rust.. but the len can change.. on each pass..
      for _ in 0..queue.len() {
          // look at each hill starting from the front of the queue and working to the back
          if let Some((hill, row, col)) = queue.pop_front() {

              // have we found our target? Sweet, send back the step count.  If there are multiple that match this'll be the first one we find so the shortest
              if grid.target_symbol == hill {
                // Some is an "option".. so satisfies Option<u32>
                return Some(steps);
              }

              // check all four directions around the point we are at.
              // for each direction, if we find a hill that is 1 shorter than the current hill
              // we will add it to the back of the queue for later inspection.
              for (dr, dc) in DIR.iter().copied() {
                  let row_x = row as isize + dr;
                  let col_x = col as isize + dc;

                  // if we leave the grid at the top/left skip this direction, its a border
                  if row_x < 0 || col_x < 0 {
                      continue;
                  }

                  // discovered I can reuse variable names and basicallly throw the old definition away
                  // so we go from isize to usize here..
                  let (row_x, col_x) = (row_x as usize, col_x as usize);

                  // if we leave the grid at the bottom/right skip this direction, it's a border
                  if row_x >= grid.points.len() || col_x >= grid.points[row_x].len() {
                      continue;
                  }

                  // we need to know what the hill is.
                  let mut new_hill = grid.points[row_x][col_x];

                  //in part 1 of the puzzle we have a special end point, convert it to height of `a`
                  new_hill = if new_hill == b'S' { b'a' } else { new_hill };

                  // skip already visited hills AND
                  // skip paths that violate the condition of maximum 1 diff in the steepness decrease
                  // from hill to new_hill
                  if new_hill == 0 || (new_hill < hill && (hill - new_hill) > 1) {
                      continue;
                  }

                  // add the hill to the back of the queue to process later..
                  queue.push_back((grid.points[row_x][col_x], row_x, col_x));

                  // mark as visited; make sure we don't go into some kind of searching loop. smart.
                  grid.points[row_x][col_x] = 0;
              }
          }
      }
      /*
      the queue changes lengths a lot.. going up and down in length.
      but each time this for statement is evaluated the queue.len() changes.

      So first pass it is a 1 so it loops one time.. The second then gets here and increments steps
      the second pass the queue len is a new value (maybe still 1; but a new 1 in the queue).
      and it loops over that queue, and then increments this again..
      */

      steps += 1;
  }

  // the goal was not met.. we found nothing.
  // None is a special Option.  I think we would use error handling against this in the caller if necessary
  None
}

#[derive(Clone, Debug)]
struct Grid {
  points: Vec<Vec<u8>>,
  target_symbol: u8,
  start_symbol: u8
}