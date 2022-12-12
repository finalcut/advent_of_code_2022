use itertools::Itertools;

fn main() {
    let totals: Vec<u64> = include_str!("../values.txt") // reads in the whole file into a single line
        .split("\n\n") // creates a vector of groups of number strings
        .map(|elf| {
            elf.split('\n') // creates a vector of numbers from each line in the group
                .map(|food| food.parse::<u64>().unwrap_or(0)).sum() // map converts the string numbers to numbers, After mapping, sum the vector up
        }) // our vector of groups of number strings is now a vector of u64 (sums)
        .sorted().rev().collect();

    println!("Part 1: {}", totals[0]);
    println!("Part 2: {}", totals.iter().take(3).sum::<u64>());
}