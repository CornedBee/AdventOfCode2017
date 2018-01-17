use std::io::{self, BufRead};

fn get_input() -> Vec<i32> {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let result : Vec<_> = stdin.lines().map(|l| l.expect("could not read stdin").parse().expect("could not parse i32")).collect();
    if result.len() >= i32::max_value() as usize {
        panic!("input too long");
    }
    result
}

fn solve1(data: &mut [i32]) -> i32 {
    let mut ip : i32 = 0;
    let mut steps = 0;
    while ip >= 0 && ip < data.len() as i32 {
        let jump = data[ip as usize];
        data[ip as usize] = jump + 1;
        ip = ip + jump;
        steps = steps + 1;
    }
    steps
}

fn solve2(data: &mut [i32]) -> i32 {
    let mut ip : i32 = 0;
    let mut steps = 0;
    while ip >= 0 && ip < data.len() as i32 {
        let jump = data[ip as usize];
        data[ip as usize] = if jump >= 3 { jump - 1 } else { jump + 1 };
        ip = ip + jump;
        steps = steps + 1;
    }
    steps
}

fn main() {
    let mut data1 = get_input();
    let mut data2 = data1.clone();
    let result1 = solve1(&mut data1);
    println!("Solution 1: {}", result1);
    let result2 = solve2(&mut data2);
    println!("Solution 2: {}", result2);
}
