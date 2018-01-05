use std::io::{self, BufRead};

struct Scanner {
    depth: u32,
    range: u32,
}

impl Scanner {
    fn hits(&self) -> bool {
        self.depth % self.period() == 0
    }

    fn period(&self) -> u32 {
        2 * self.range - 2
    }

    fn severity(&self) -> u32 {
        self.depth * self.range
    }
}

fn get_input() -> Vec<Scanner> {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let result = stdin.lines()
        .map(|l| {
            let l = l.expect("could not read stdin");
            let mut ints = l.split(':').map(|s| s.trim().parse().expect("could not parse integer"));
            Scanner {depth: ints.next().expect("missing depth"), range: ints.next().expect("missing range")}
        })
        .collect();
    result
}

fn solve(data: &[Scanner]) -> u32 {
    data.iter().filter(|s| s.hits()).fold(0, |sum, s| sum + s.severity())
}

fn main() {
    let data = get_input();
    let result = solve(&data);
    println!("Solution: {}", result);
}
