use std::io::{self, Read};

fn get_input() -> Vec<u32> {
    let mut data = String::new();
    io::stdin().read_to_string(&mut data).expect("Couldn't read stdin");
    data.chars().flat_map(|c| c.to_digit(10)).collect()
}

fn solve(numbers: &[u32]) -> u32 {
    numbers.iter().zip(numbers.iter().cycle().skip(1))
        .filter(|&(a, b)| a == b)
        .fold(0, |sum, (a, _)| sum + a)
}

fn main() {
    let numbers = get_input();
    println!("Solution: {}", solve(&numbers));
}
