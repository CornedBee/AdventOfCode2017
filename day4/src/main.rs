use std::collections::HashSet;
use std::io::{self, BufRead};

fn get_input() -> Vec<String> {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let result = stdin.lines().collect::<Result<_, _>>().expect("Could not read input");
    result
}

struct Passphrase<'a> {
    words: Vec<&'a str>,
}

impl <'a> Passphrase<'a> {
    fn from(s: &'a str) -> Self {
        Passphrase { words: s.split_whitespace().collect() }
    }

    fn is_valid(&self) -> bool {
        self.words.len() == self.words.iter().collect::<HashSet<_>>().len()
    }
}

fn solve(data: &[String]) -> usize {
    data.into_iter().map(|s| Passphrase::from(s)).filter(|p| p.is_valid()).count()
}

fn main() {
    let data = get_input();
    let result = solve(&data);
    println!("Solution: {}", result);
}
