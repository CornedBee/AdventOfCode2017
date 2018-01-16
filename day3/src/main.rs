use std::collections::HashMap;
use std::io::{self, Read};

fn get_input() -> u32 {
    let mut data = String::new();
    io::stdin().read_to_string(&mut data).expect("Couldn't read stdin");
    data.trim().parse().expect("Bad input")
}

fn layer(cell: u32) -> u32 {
    (((cell - 1) as f64).sqrt().floor() as u32 + 1) / 2
}

fn solve1(cell: u32) -> u32 {
    if cell == 1 { return 0; }

    let layer = layer(cell);
    println!("Layer: {}", layer);
    let layer_base = layer * layer;
    let layer_side = layer * 2 + 1;
    println!("Layer side: {}", layer_side);
    let layer_wobble = (layer_side + 1) / 2;
    println!("Layer wobble: {}", layer_wobble);
    let offset_in_layer = cell - layer_base - 1;
    println!("Offset in layer: {}", offset_in_layer);
    let offset_from_bottom_right = offset_in_layer + 1;
    println!("Offset from bottom right: {}", offset_from_bottom_right);
    let wobbled_offset = offset_from_bottom_right % 2 * layer_wobble;
    println!("Wobbled offset: {}", wobbled_offset);
    
    let offset_from_center = offset_from_bottom_right % ((layer_side + 1) / 2);
    println!("Offset from center: {}", offset_from_center);
    layer + offset_from_center
}

fn solve2(threshold: u32) -> u32 {
    // We're just going to brute-force this.
    let mut memory = HashMap::new();
    memory.insert((0, 0), 1);
    let mut ring = 1;
    loop {
        for i in get_indices(ring) {
            let value = sum_around(&memory, i);
            if value > threshold {
                return value;
            }
            memory.insert(i, value);
        }
        ring += 1;
    }
}

fn get_indices(ring: i32) -> Vec<(i32, i32)> {
    let side = ring * 2 + 1;
    let slides = [(( ring, -ring), ( 0,  1)),
                  (( ring,  ring), (-1,  0)),
                  ((-ring,  ring), ( 0, -1)),
                  ((-ring, -ring), ( 1,  0))];
    let mut result = vec![];
    for &(mut pos, stride) in slides.into_iter() {
        for _ in 1..side { // want one less iteration than side length
            pos.0 += stride.0;
            pos.1 += stride.1;
            result.push(pos);
        }
    }
    result
}

fn sum_around(memory: &HashMap<(i32, i32), u32>, index: (i32, i32)) -> u32 {
    indices_around(index).into_iter()
        .flat_map(|i| memory.get(i))
        .fold(0, |sum, v| sum + v)
}

fn indices_around(index: (i32, i32)) -> [(i32, i32); 8] {
    let (x, y) = index;
    [(x-1, y+1), (x  , y+1), (x+1, y+1),
     (x-1, y  ),             (x+1, y  ),
     (x-1, y-1), (x  , y-1), (x+1, y-1)]
}

fn main() {
    let data = get_input();
    println!("Solution Part A: {}", solve1(data));
    println!("Solution Part B: {}", solve2(data));
}

#[cfg(test)]
mod tests {
    use super::solve1;
    use super::solve2;

    #[test] #[ignore]
    fn check1() {
        assert_eq!(0, solve1(1));
        assert_eq!(1, solve1(2));
        assert_eq!(2, solve1(3));
        assert_eq!(1, solve1(4));
        assert_eq!(2, solve1(5));
        assert_eq!(1, solve1(6));
        assert_eq!(2, solve1(7));
        assert_eq!(1, solve1(8));
        assert_eq!(2, solve1(9));
        assert_eq!(3, solve1(10));
        assert_eq!(2, solve1(11));
        assert_eq!(3, solve1(12));
        assert_eq!(4, solve1(13));
        assert_eq!(3, solve1(14));
        assert_eq!(2, solve1(15));
        assert_eq!(3, solve1(16));
        assert_eq!(4, solve1(17));
        assert_eq!(3, solve1(18));
        assert_eq!(2, solve1(19));
        assert_eq!(3, solve1(20));
        assert_eq!(4, solve1(21));
        assert_eq!(3, solve1(22));
        assert_eq!(2, solve1(23));
        assert_eq!(3, solve1(24));
        assert_eq!(4, solve1(25));
        assert_eq!(5, solve1(26));
        assert_eq!(4, solve1(27));
        assert_eq!(3, solve1(28));
        assert_eq!(4, solve1(29));
        assert_eq!(5, solve1(30));
        assert_eq!(6, solve1(30));
        assert_eq!(5, solve1(30));
    }

    #[test]
    fn check2() {
        assert_eq!(4, solve2(3));
        assert_eq!(10, solve2(9));
        assert_eq!(11, solve2(10));
        assert_eq!(54, solve2(26));
        assert_eq!(59, solve2(58));
        assert_eq!(147, solve2(142));
        assert_eq!(304, solve2(147));
    }
}