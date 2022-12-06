use aoc_util::{get_seed_data, split_delimited};

fn main() -> std::io::Result<()> {
    let input = get_seed_data().expect("Could not load values");

    let output = split_delimited(&input, &"".to_owned());

    let mut sum_vec: Vec<u32> = Vec::with_capacity(output.len());

    for &i in &output {
        let number_vec: Vec<u32> = i.iter().flat_map(|x| x.parse()).collect(); // converts each of the sub vectors into numbers from strings and removes any values that can't become numbers (blanks)
        let sum: u32 = number_vec.iter().sum();
        sum_vec.push(sum);
    }

    sum_vec.sort();
    sum_vec.reverse();

    println!("Richest Elf: {:?}", sum_vec[0]);

    sum_vec.truncate(3);
    let top3: u32 = sum_vec.iter().sum();
    println!("Top Three Elves: {:?}", top3);

    Ok(())
}
