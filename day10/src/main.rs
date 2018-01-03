#![feature(inclusive_range_syntax)]

use std::io::{self, Read};

fn get_input() -> Vec<usize> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("could not read stdin");
    buffer.trim().split(',').map(|s| s.parse().expect("could not parse u32")).collect()
}

fn solve(spans: &[usize], memory_size: usize) -> usize {
    let mut memory: Vec<_> = (0..memory_size).collect();
    let mut position = 0;

    for (skip, &span) in spans.iter().enumerate() {
        tie_knot(&mut memory, position, span);
        position += skip + span;
        position %= memory.len();
    }

    memory[0] * memory[1]
}

fn tie_knot(memory: &mut [usize], start: usize, length: usize) {
    for i in 0..(length/2) {
        let a = (start + i) % memory.len();
        let b = (start + length - i - 1) % memory.len();
        memory.swap(a, b);
    }
}

fn main() {
    let data = get_input();
    let result = solve(&data, 256);
    println!("Solution: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(12, solve(&[3, 4, 1, 5], 5));
    }
}
