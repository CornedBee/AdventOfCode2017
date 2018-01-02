#[macro_use] extern crate scan_rules;

use std::collections::HashSet;
use std::io;
use scan_rules::scanner::Word;

struct Program {
    name: String,
    children: Vec<String>,
}

fn read_program() -> Result<Option<Program>, scan_rules::ScanError> {
    let mut line = String::new();
    match io::stdin().read_line(&mut line) {
        Err(e) => Err(scan_rules::ScanError::io(e)),
        Ok(0) => Ok(None),
        Ok(_) =>
            Ok(Some(scan!(&line;
                (let name: Word<String>, "(", let _: u32, ")", ["->", [ let children : Word<String> ],+ ]?) =>
                    Program { name, children: children.into_iter().next().unwrap_or(vec![]) },
            )?)),
    }
}

fn get_input() -> Vec<Program> {
    let mut result = Vec::new();
    loop {
        match read_program().expect("failed to parse program") {
            None => break,
            Some(p) => result.push(p),
        }
    }
    result
}

fn solve(data: &[Program]) -> &str {
    let programs : HashSet<&str> = data.iter().map(|p| p.name.as_str()).collect();
    let children : HashSet<&str> = data.iter().flat_map(|p| p.children.iter().map(|c| c.as_str())).collect();

    let mut roots = programs.difference(&children);
    let root = roots.next().expect("no root programs");
    if !roots.next().is_none() {
        panic!("more than one root program");
    }
    root
}

fn main() {
    let data = get_input();
    let result = solve(&data);
    println!("Solution: {}", result);
}
