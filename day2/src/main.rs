use std::io::{self, BufRead};

fn parse_line(line: &str) -> Vec<u32> {
    line.split_whitespace().flat_map(|w| w.parse().ok()).collect()
}

fn get_input() -> Vec<Vec<u32>> {
    let stdin = io::stdin();
    let result : Vec<_> = stdin.lock().lines().flat_map(|l| l.ok()).map(|l| parse_line(&l)).collect();
    result
}

fn sort_lines(data: &mut [Vec<u32>]) {
    for v in data {
        v.sort_unstable_by(|a, b| b.cmp(a));
    }
}

fn solve1(sheet: &[Vec<u32>]) -> u32 {
    sheet.iter().map(|line| line.first().unwrap() - line.last().unwrap()).fold(0, |sum, i| sum + i)
}

fn solve2(sheet: &[Vec<u32>]) -> u32 {
    sheet.iter().map(|line| even_division(line)).fold(0, |sum, i| sum + i)
}

fn even_division(line: &[u32]) -> u32 {
    for i in 0..line.len()-1 {
        for j in i+1..line.len() {
            if line[i] % line[j] == 0 {
                return line[i] / line[j];
            }
        }
    }
    panic!("line didn't have evenly divisible pair");
}

fn main() {
    let mut data = get_input();
    sort_lines(&mut data);
    println!("Solution Part A: {}", solve1(&data));
    println!("Solution Part B: {}", solve2(&data));
}
