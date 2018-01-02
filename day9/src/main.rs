use std::io::{self, Read};

fn get_input() -> String {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("could not read stdin");
    buffer
}

enum State {
    Good,
    Garbage,
    GarbageEscape,
}

fn solve(data: &str) -> u32 {
    let mut score = 0;
    let mut depth = 0;
    let mut state = State::Good;
    for (o, c) in data.chars().enumerate() {
        match (state, c) {
            (State::Good, '{') => {
                depth += 1;
                state = State::Good;
            },
            (State::Good, '}') => {
                if depth == 0 {
                    panic!("Unexpected close brace");
                }
                score += depth;
                depth -= 1;
                state = State::Good;
            },
            (State::Good, '<') => {
                state = State::Garbage;
            },
            (State::Good, ',') => {
               state = State::Good;
            }
            (State::Good, c) => {
                panic!("Unexpected input: {}", c);
            },
            (State::Garbage, '>') => {
                state = State::Good;
            },
            (State::Garbage, '!') => {
                state = State::GarbageEscape;
            },
            (_, c) => {
                state = State::Garbage;
            }
        }
    }
    if depth != 0 {
        panic!("Unmatched open brace ({})", depth);
    }
    score
}

fn main() {
    let data = get_input();
    let result = solve(data.trim());
    println!("Solution: {}", result);
}
