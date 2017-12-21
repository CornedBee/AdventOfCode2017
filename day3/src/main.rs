use std::io::{self, Read};

fn get_input() -> u32 {
    let mut data = String::new();
    io::stdin().read_to_string(&mut data).expect("Couldn't read stdin");
    data.trim().parse().expect("Bad input")
}

fn layer(cell: u32) -> u32 {
    (((cell - 1) as f64).sqrt().floor() as u32 + 1) / 2
}

fn solve(cell: u32) -> u32 {
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
     ((offset_from_bottom_center % layer_side) as i32 - (layer_side / 2) as i32).abs() as u32;
    println!("Offset from center: {}", offset_from_center);
    layer + offset_from_center
}

fn main() {
    let cell = get_input();
    println!("Solution: {}", solve(cell));
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn check() {
        assert_eq!(0, solve(1));
        assert_eq!(1, solve(2));
        assert_eq!(2, solve(3));
        assert_eq!(1, solve(4));
        assert_eq!(2, solve(5));
        assert_eq!(1, solve(6));
        assert_eq!(2, solve(7));
        assert_eq!(1, solve(8));
        assert_eq!(2, solve(9));
        assert_eq!(3, solve(10));
        assert_eq!(2, solve(11));
        assert_eq!(3, solve(12));
        assert_eq!(4, solve(13));
        assert_eq!(3, solve(14));
        assert_eq!(2, solve(15));
        assert_eq!(3, solve(16));
        assert_eq!(4, solve(17));
        assert_eq!(3, solve(18));
        assert_eq!(2, solve(19));
        assert_eq!(3, solve(20));
        assert_eq!(4, solve(21));
        assert_eq!(3, solve(22));
        assert_eq!(2, solve(23));
        assert_eq!(3, solve(24));
        assert_eq!(4, solve(25));
        assert_eq!(5, solve(26));
        assert_eq!(4, solve(27));
        assert_eq!(3, solve(28));
        assert_eq!(4, solve(29));
        assert_eq!(5, solve(30));
        assert_eq!(6, solve(30));
        assert_eq!(5, solve(30));
    }
}