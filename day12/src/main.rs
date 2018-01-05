extern crate disjoint_sets;
#[macro_use] extern crate scan_rules;

use std::io;

struct Program {
    id: usize,
    connections: Vec<usize>,
}

fn read_program() -> Result<Option<Program>, scan_rules::ScanError> {
    let mut line = String::new();
    match io::stdin().read_line(&mut line) {
        Err(e) => Err(scan_rules::ScanError::io(e)),
        Ok(0) => Ok(None),
        Ok(_) =>
            Ok(Some(scan!(&line;
                (let id, "<->", [ let connections: usize ],+) =>
                    Program { id, connections },
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

fn make_unions(data: &[Program]) -> disjoint_sets::UnionFind {
    let set_size = data.iter().map(|p| p.id).max().expect("no programs") + 1;
    let mut union_find = disjoint_sets::UnionFind::new(set_size);

    for p in data {
        for c in &p.connections {
            union_find.union(p.id, *c);
        }
    }

    union_find
}

fn solve(data: &[Program]) -> usize {
    let union_find = make_unions(data);

    data.iter().filter(|p| union_find.equiv(0, p.id)).count()
}

fn main() {
    let data = get_input();
    let result = solve(&data);
    println!("Solution: {}", result);
}
