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

fn solve(data: &str) -> (u32, u32) {
    let mut score = 0;
    let mut depth = 0;
    let mut state = State::Good;
    let mut garbage_amount = 0;
    for c in data.chars() {
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
            (State::Garbage, _) => {
                garbage_amount += 1;
                state = State::Garbage;
            }
            (State::GarbageEscape, _) => {
                state = State::Garbage;
            }
        }
    }
    if depth != 0 {
        panic!("Unmatched open brace ({})", depth);
    }
    (score, garbage_amount)
}

fn main() {
    let data = get_input();
    let result = solve(data.trim());
    println!("Solution 1: {}", result.0);
    println!("Solution 2: {}", result.1);
}
