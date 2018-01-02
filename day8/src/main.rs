#[macro_use] extern crate scan_rules;

use std::collections::HashMap;
use std::io;
use scan_rules::scanner::{Word, NonSpace};

struct Instruction {
    register: String,
    code: InstructionCode,
    value: i32,
    condition: Condition,
}

impl Instruction {
    fn execute(&self, state: &mut State) {
        if self.condition.evaluate(state) {
            self.do_execute(state);
        }
    }

    fn do_execute(&self, state: &mut State) {
        state.modify_register(&self.register,
            |v| self.code.execute(v, self.value));
    }
}

enum InstructionCode {
    Inc,
    Dec,
}

impl InstructionCode {
    fn parse(v: &str) -> InstructionCode {
        match v {
            "inc" => InstructionCode::Inc,
            "dec" => InstructionCode::Dec,
            _ => panic!("unknown instruction code"),
        }
    }

    fn execute(&self, state: &mut i32, value: i32) {
        match *self {
            InstructionCode::Inc => *state = *state + value,
            InstructionCode::Dec => *state = *state - value,
        }
    }
}

struct Condition {
    register: String,
    operation: ComparisonOp,
    value: i32,
}

impl Condition {
    fn evaluate(&self, state: &State) -> bool {
        self.operation.evaluate(state.get_register(&self.register), self.value)
    }
}

enum ComparisonOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

impl ComparisonOp {
    fn parse(v: &str) -> ComparisonOp {
        match v {
            "==" => ComparisonOp::Eq,
            "!=" => ComparisonOp::Ne,
            "<=" => ComparisonOp::Le,
            "<" => ComparisonOp::Lt,
            ">=" => ComparisonOp::Ge,
            ">" => ComparisonOp::Gt,
            _ => panic!("unknown comparison operation"),
        }
    }

    fn evaluate(&self, left: i32, right: i32) -> bool {
        match *self {
            ComparisonOp::Eq => left == right,
            ComparisonOp::Ne => left != right,
            ComparisonOp::Lt => left < right,
            ComparisonOp::Le => left <= right,
            ComparisonOp::Gt => left > right,
            ComparisonOp::Ge => left >= right,
        }
    }
}

fn read_instruction() -> Result<Option<Instruction>, scan_rules::ScanError> {
    let mut line = String::new();
    match io::stdin().read_line(&mut line) {
        Err(e) => Err(scan_rules::ScanError::io(e)),
        Ok(0) => Ok(None),
        Ok(_) =>
            Ok(Some(scan!(&line;
                (let register: Word<String>, let code: Word<String>, let value,
                    "if", let cmp_register: Word<String>, let cmp_operation: NonSpace<String>, let cmp_value) =>
                    Instruction { register, code: InstructionCode::parse(&code), value,
                        condition: Condition { register: cmp_register, operation: ComparisonOp::parse(&cmp_operation), value: cmp_value } },
            )?)),
    }
}

fn get_input() -> Vec<Instruction> {
    let mut result = Vec::new();
    loop {
        match read_instruction().expect("failed to parse instruction") {
            None => break,
            Some(p) => result.push(p),
        }
    }
    result
}

struct State {
    registers: HashMap<String, i32>,
}

impl State {
    fn new() -> Self {
        Self { registers: HashMap::new() }
    }

    fn get_register(&self, key: &str) -> i32 {
        *self.registers.get(key).unwrap_or(&0)
    }

    fn modify_register<F: FnOnce(&mut i32)>(&mut self, key: &str, f: F) {
        f(self.registers.entry(key.to_owned()).or_insert(0));
    }

    fn largest_register(&self) -> i32 {
        self.registers.values().cloned().max().unwrap_or(0)
    }
}

fn solve(data: &[Instruction]) -> i32 {
    let mut state = State::new();
    for i in data {
        i.execute(&mut state);
    }
    state.largest_register()
}

fn main() {
    let data = get_input();
    let result = solve(&data);
    println!("Solution: {}", result);
}
