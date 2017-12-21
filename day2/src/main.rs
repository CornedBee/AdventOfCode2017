use std::io::{self, BufRead};

fn parse_line(line: &str) -> Vec<u32> {
    line.split_whitespace().flat_map(|w| w.parse().ok()).collect()
}

fn get_input() -> Vec<Vec<u32>> {
    let stdin = io::stdin();
    let result : Vec<_> = stdin.lock().lines().flat_map(|l| l.ok()).map(|l| parse_line(&l)).collect();
    result
}

fn solve(sheet: &[Vec<u32>]) -> u32 {
    sheet.iter().map(|line| line.iter().max().unwrap() - line.iter().min().unwrap()).fold(0, |sum, i| sum + i)
}

fn main() {
    let data = get_input();
    println!("Solution: {}", solve(&data));
}
