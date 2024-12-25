use d15::{P1ObjectKind, Warehouse, input::read_input, solve};

const PATH: &str = "d15/input/input.txt";

fn main() {
    let (warehouse, moves): (Warehouse<P1ObjectKind>, _) = read_input(PATH);
    solve(warehouse, &moves);
}
