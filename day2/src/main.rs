#![feature(universal_impl_trait)]

use std::io::{self, BufRead};

fn parse_line(line: &str) -> Vec<u32> {
    line.split_whitespace().flat_map(|w| w.parse().ok()).collect()
}

fn get_input() -> Vec<Vec<u32>> {
    let stdin = io::stdin();
    let result : Vec<_> = stdin.lock().lines().flat_map(|l| l.ok()).map(|l| parse_line(&l)).collect();
    result
}

fn solve(sheet: impl IntoIterator<Item = &Vec<u32>>) -> u32 {
    
}

fn main() {
    println!("Hello, world!");
}
