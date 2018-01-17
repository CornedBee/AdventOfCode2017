#[macro_use] extern crate scan_rules;

use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use scan_rules::scanner::Word;

struct Program {
    name: String,
    weight: i32,
    children: Vec<String>,
}

fn read_program() -> Result<Option<Program>, scan_rules::ScanError> {
    let mut line = String::new();
    match io::stdin().read_line(&mut line) {
        Err(e) => Err(scan_rules::ScanError::io(e)),
        Ok(0) => Ok(None),
        Ok(_) =>
            Ok(Some(scan!(&line;
                (let name: Word<String>, "(", let weight: i32, ")", ["->", [ let children : Word<String> ],+ ]?) =>
                    Program { name, weight, children: children.into_iter().next().unwrap_or(vec![]) },
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

fn solve1(data: &[Program]) -> &str {
    let programs : HashSet<&str> = data.iter().map(|p| p.name.as_str()).collect();
    let children : HashSet<&str> = data.iter().flat_map(|p| p.children.iter().map(|c| c.as_str())).collect();

    let mut roots = programs.difference(&children);
    let root = roots.next().expect("no root programs");
    if !roots.next().is_none() {
        panic!("more than one root program");
    }
    root
}

enum FindImbalanceResult {
    FoundImbalance(i32),
    CumulativeWeight(i32),
}

fn solve2(data: &[Program], root: &str) -> i32 {
    let programs = data.iter().map(|p| (p.name.as_str(), p)).collect();
    match find_imbalance(&programs, root) {
        FindImbalanceResult::FoundImbalance(imbalance) => imbalance,
        FindImbalanceResult::CumulativeWeight(_) => panic!("no imbalance found")
    }
}

fn find_imbalance(programs: &HashMap<&str, &Program>, node: &str) -> FindImbalanceResult {
    let program = programs[node];
    let mut cumulative_weight = program.weight;
    let mut weights = HashMap::<i32, Vec<&str>>::new();
    for c in &program.children {
        let weight = match find_imbalance(programs, &c) {
            FindImbalanceResult::CumulativeWeight(w) => w,
            r => return r,
        };
        cumulative_weight += weight;
        weights.entry(weight).or_insert_with(|| vec![]).push(c);
    }

    if weights.len() <= 1 {
        return FindImbalanceResult::CumulativeWeight(cumulative_weight);
    }

    let odd = weights.iter().find(|e| e.1.len() == 1).expect("can't find lone entry");
    let odd_weight = odd.0;
    let odd_name = odd.1[0];
    let expected_weight = weights.iter().find(|e| e.1.len() != 1).expect("can't find standard entry").0;
    let adjustment = expected_weight - odd_weight;
    let odd_own_weight = programs[odd_name].weight;

    FindImbalanceResult::FoundImbalance(odd_own_weight + adjustment)
}

fn main() {
    let data = get_input();
    let result = solve1(&data);
    println!("Solution 1: {}", result);
    let result = solve2(&data, result);
    println!("Solution 2: {}", result);
}
