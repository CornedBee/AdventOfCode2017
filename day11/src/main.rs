use std::io::{self, Read};

#[macro_use]
extern crate enum_map;

use enum_map::EnumMap;

#[derive(Copy, Clone, Debug, EnumMap)]
pub enum Axis {
    Nw, N, Ne,
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Forward,
    Backward,
}

#[derive(Copy, Clone, Debug)]
pub struct Step {
    pub axis: Axis,
    pub direction: Direction,
}

impl Step {
    fn parse(v: &str) -> Step {
        use Axis::*;
        use Direction::*;
        match v {
            "nw" => Step { axis: Nw, direction: Forward },
            "n" => Step { axis: N, direction: Forward },
            "ne" => Step { axis: Ne, direction: Forward },
            "sw" => Step { axis: Ne, direction: Backward },
            "s" => Step { axis: N, direction: Backward },
            "se" => Step { axis: Nw, direction: Backward },
            _ => panic!("unknown step code {}", v),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Travel(EnumMap<Axis, i32>);

impl Travel {
    pub fn new() -> Travel {
        Travel(EnumMap::new())
    }

    pub fn take(&mut self, step: Step) {
        use Direction::*;
        self.0[step.axis] +=
            match step.direction { Forward => 1, Backward => -1 };
    }

    pub fn reduce(&mut self) {
        use Axis::*;

        self.merge(Nw, Ne, 1, N);
        self.merge(N, Ne, -1, Nw);
        self.merge(N, Nw, -1, Ne);
    }

    fn merge(&mut self, a1: Axis, a2: Axis, a2_mod: i32, at: Axis) {
        let v1 = self.0[a1];
        let a1_signum = v1.signum();
        let v2 = self.0[a2] * a2_mod;
        if a1_signum == v2.signum() {
            let vt = v1.abs().min(v2.abs()) * a1_signum;
            self.0[a1] -= vt;
            self.0[a2] -= vt * a2_mod;
            self.0[at] += vt;
        }
    }

    pub fn step_count(&self) -> u32 {
        self.0.values().fold(0, |sum, v| sum + (v.abs() as u32))
    }
}

fn get_input() -> Travel {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("could not read stdin");
    let mut result = Travel::new();
    for s in buffer.trim().split(',').map(|s| Step::parse(s)) {
        result.take(s);
    }
    result
}

fn solve(mut data: Travel) -> u32 {
    data.reduce();
    data.step_count()
}

fn main() {
    let data = get_input();
    let result = solve(data);
    println!("Solution: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    use Axis::*;
    use Direction::*;

    #[test]
    fn counts_trivial_path() {
        let mut path = Travel::new();
        path.take(Step { axis: N, direction: Forward });
        path.take(Step { axis: N, direction: Forward });

        path.reduce();

        assert_eq!(2, path.step_count());
    }

    #[test]
    fn reduces_opposing_steps() {
        let mut path = Travel::new();
        path.take(Step { axis: N, direction: Forward });
        path.take(Step { axis: N, direction: Backward });

        path.reduce();

        assert_eq!(0, path.step_count());
    }

    #[test]
    fn reduces_n_sw() {
        let mut path = Travel::new();
        path.take(Step { axis: N, direction: Forward });
        path.take(Step { axis: Ne, direction: Backward });

        path.reduce();

        assert_eq!(1, path.step_count());
    }

    #[test]
    fn reduces_n_se() {
        let mut path = Travel::new();
        path.take(Step { axis: N, direction: Forward });
        path.take(Step { axis: Nw, direction: Backward });

        path.reduce();

        assert_eq!(1, path.step_count());
    }

    #[test]
    fn reduces_s_ne() {
        let mut path = Travel::new();
        path.take(Step { axis: N, direction: Backward });
        path.take(Step { axis: Nw, direction: Forward });

        path.reduce();

        assert_eq!(1, path.step_count());
    }

    #[test]
    fn reduces_ne_nw() {
        let mut path = Travel::new();
        path.take(Step { axis: Nw, direction: Forward });
        path.take(Step { axis: Ne, direction: Forward });

        path.reduce();

        assert_eq!(1, path.step_count());
    }

    #[test]
    fn reduces_se_sw() {
        let mut path = Travel::new();
        path.take(Step { axis: Nw, direction: Backward });
        path.take(Step { axis: Ne, direction: Backward });

        path.reduce();

        assert_eq!(1, path.step_count());
    }
}
