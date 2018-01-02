use std::collections::HashSet;
use std::io::{self, Read};

fn get_input() -> Vec<u32> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("could not read stdin");
    buffer.split_whitespace().map(|s| s.parse()).collect::<Result<_, _>>().expect("could not parse number")
}

fn redistribute(data: &mut [u32]) {
    let (target, &blocks) = data.iter().enumerate()
        .max_by(|&(i1, blocks1), &(i2, blocks2)| blocks1.cmp(&blocks2).then(i1.cmp(&i2).reverse())).unwrap();
    data[target] = 0;
    let len = data.len();
    let all = blocks / (len as u32);
    let special = (blocks as usize) % len;
    for i in 0..special {
        let loc = &mut data[(target + 1 + i) % len];
        *loc = *loc + all + 1;
    }
    for i in special..len {
        let loc = &mut data[(target + 1 + i) % len];
        *loc = *loc + all;
    }
}

fn solve(data: &mut [u32]) -> u32 {
    let mut result = 0;
    let mut seen = HashSet::new();
    loop {
        let current = data.to_vec();
        if !seen.insert(current) {
            return result;
        }

        redistribute(data);
        result = result + 1;
    }
}

fn main() {
    let mut data = get_input();
    let result = solve(&mut data);
    println!("Solution: {}", result);
}
