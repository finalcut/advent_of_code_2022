use aoc_util::read_file;

#[derive(Debug, Clone)]
struct Signal {
    during: i64,
    after: i64,
}

fn main() {
    let data = read_file("values.txt").expect("Could not load values");

    let mut x_register_value: Vec<i64> = Vec::new(); // tracking register value at the end of each cycle
    let mut signal_strength: Vec<Signal> = Vec::new();

    x_register_value.push(1);
    signal_strength.push(Signal {
        during: 0,
        after: 0,
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
            after: s1,
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
                after: compute_strength(r2, cycle),
            };

            //println!("ADDR cycle: {}, x: {}, D: {}, A: {}", cycle, r2, sig.during, sig.after);

            signal_strength.push(sig);
            x_register_value.push(r2);
        }
    }

    let part1 = sum_cycles(&signal_strength, [20, 60, 100, 140, 180, 220].to_vec());
    println!("part1: {:?}", part1);
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
