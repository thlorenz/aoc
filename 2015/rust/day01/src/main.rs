// https://adventofcode.com/2015/day/1
fn main() {
    let input = include_bytes!("input.txt");

    let (_, floor, basement_idx): (usize, i32, Option<usize>) =
        input
            .into_iter()
            .fold((1, 0, None), |(idx, floor, basement_idx), c| {
                let floor = match c {
                    40 => floor + 1,
                    41 => floor - 1,
                    _ => floor,
                };
                let basement_idx = if floor < 0 && basement_idx.is_none() {
                    Some(idx)
                } else {
                    basement_idx
                };
                (idx + 1, floor, basement_idx)
            });

    println!(
        "Floor: {}, Basement idx: {:?}",
        floor,
        basement_idx.expect("Should have found basement idx")
    );
}
