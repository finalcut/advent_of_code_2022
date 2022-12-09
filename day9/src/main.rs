use aoc_util::read_file;
fn main() {
    let moves = read_file("test1.txt").expect("Could not load values");

    let mut two_knots: Vec<[i16; 2]> = Vec::new();

    /*
    2  . . . .
    1  . H . .
    0  T . . .
    -1 . . . .
    */

    let mut h = [0, 0];
    let mut t = [0, 0];
    two_knots.push(t.clone());
    let dist_limit = 2 as i16;

    for line in moves {
        let move_info: Vec<&str> = line.split_whitespace().collect();
        let dir = move_info[0];
        let dist = move_info[1].parse().unwrap();
        println!("==============");
        match dir {
            "U" => move_up(&mut h, &mut t, dist, &mut two_knots, dist_limit),
            "D" => move_down(&mut h, &mut t, dist, &mut two_knots, dist_limit),
            "L" => move_left(&mut h, &mut t, dist, &mut two_knots, dist_limit),
            "R" => move_right(&mut h, &mut t, dist, &mut two_knots, dist_limit),
            _ => println!("NOT MOVING"),
        }
    }

    println!("{:?}", two_knots);

    two_knots.sort_unstable();
    two_knots.dedup();


    println!("spots visited: {:?}", two_knots.len());
}

fn move_up(
    h: &mut [i16; 2],
    t: &mut [i16; 2],
    dist: i16,
    visits: &mut Vec<[i16; 2]>,
    dist_limit: i16,
) {
    println!("moving up: {}", dist);
    for _i in 0..dist {
        h[1] += 1;
        let dist_b = distance_between(h, t);
        println!("{} diff: {} between h {:?} and t {:?}", _i, dist_b, h, t);
        if dist_b > 1 {
            println!("gotta move t");
            if h[1] - t[1] == dist_limit {
                t[1] += 1;
            }

            let xd = h[0] - t[0];
            if xd != dist_limit - 2 {
                if xd < 0 {
                    t[0] -= 1;
                } else {
                    t[0] += 1;
                }
            }

            println!("{} final: h {:?} and t {:?}", _i, h, t);
        }
        visits.push(t.clone());
    }
}
fn move_down(
    h: &mut [i16; 2],
    t: &mut [i16; 2],
    dist: i16,
    visits: &mut Vec<[i16; 2]>,
    dist_limit: i16,
) {
    println!("moving down: {}", dist);
    for _i in 0..dist {
        h[1] -= 1;
        let dist_b = distance_between(h, t);
        println!("{} diff: {} between h {:?} and t {:?}", _i, dist_b, h, t);
        if dist_b > 1 {
            println!("gotta move t");
            if t[1] - h[1] == dist_limit {
                t[1] -= 1;
            }

            let xd = h[0] - t[0];
            if xd != dist_limit - 2 {
                if xd < 0 {
                    t[0] -= 1;
                } else {
                    t[0] += 1;
                }
            }

            println!("{} final: h {:?} and t {:?}", _i, h, t);
        }
        visits.push(t.clone());
    }
}

fn move_left(
    h: &mut [i16; 2],
    t: &mut [i16; 2],
    dist: i16,
    visits: &mut Vec<[i16; 2]>,
    dist_limit: i16,
) {
    println!("moving left: {}", dist);
    for _i in 0..dist {
        h[0] -= 1;
        let dist_b = distance_between(h, t);
        println!("{} diff: {} between h {:?} and t {:?}", _i, dist_b, h, t);
        if dist_b > 1 {
            println!("gotta move t");
            if t[0] - h[0] == dist_limit {
                t[0] -= 1;
            }

            if (h[1] - t[1]) != dist_limit - 2 {
                t[1] = h[1];
            }

            let yd = h[1] - t[1];
            if yd != dist_limit - 2 {
                if yd < 0 {
                    t[1] -= 1;
                } else {
                    t[1] += 1;
                }
            }
            println!("{} final: h {:?} and t {:?}", _i, h, t);
        }
        visits.push(t.clone());
    }
}
fn move_right(
    h: &mut [i16; 2],
    t: &mut [i16; 2],
    dist: i16,
    visits: &mut Vec<[i16; 2]>,
    dist_limit: i16,
) {
    println!("moving right: {}", dist);
    for _i in 0..dist {
        h[0] += 1;
        let dist_b = distance_between(h, t);
        println!("{} diff: {} between h {:?} and t {:?}", _i, dist_b, h, t);
        if dist_b > 1 {
            println!("gotta move t {}", dist_b);
            if h[0] - t[0] == dist_limit {
                t[0] += 1;
            }

            let yd = h[1] - t[1];
            if yd != dist_limit - 2 {
                if yd < 0 {
                    t[1] -= 1;
                } else {
                    t[1] += 1;
                }
            }

            println!("{} final: h {:?} and t {:?}", _i, h, t);
        }
        visits.push(t.clone());
    }
}

fn distance_between(head: &mut [i16; 2], tail: &mut [i16; 2]) -> i16 {
    let h = head.clone();
    let t = tail.clone();
    *[(h[0] - t[0]).abs(), (h[1] - t[1]).abs()]
        .iter()
        .max()
        .unwrap()
}
