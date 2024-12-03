use itertools::Itertools;

fn read_input() -> (Vec<usize>, Vec<usize>) {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b' ')
        .has_headers(false)
        .from_path("d1p1/input/input.txt")
        .unwrap();

    reader
        .deserialize()
        .map(|deser| {
            let (left, _, _, right): (usize, String, String, usize) = deser.unwrap();
            (left, right)
        })
        .unzip()
}

fn main() {
    let (lefts, rights) = read_input();
    let right_counts = rights.into_iter().counts();

    let tot: usize = lefts
        .into_iter()
        .map(|left| right_counts.get(&left).unwrap_or(&0) * left)
        .sum();

    println!("tot: {}", tot);
}
