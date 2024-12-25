use d15::{P1ObjectKind, P2ObjectKind, Point, Warehouse, input::read_input, solve};

const PATH: &str = "d15/input/input.txt";

fn double_warehouse(warehouse: Warehouse<P1ObjectKind>) -> Warehouse<P2ObjectKind> {
    let layout = warehouse
        .layout
        .into_iter()
        .map(|row| {
            row.into_iter()
                .flat_map(|obj| match obj {
                    P1ObjectKind::Wall => [P2ObjectKind::Wall, P2ObjectKind::Wall],
                    P1ObjectKind::Open => [P2ObjectKind::Open, P2ObjectKind::Open],
                    P1ObjectKind::Box => [P2ObjectKind::BoxOpen, P2ObjectKind::BoxClose],
                })
                .collect()
        })
        .collect();
    let pos = Point(warehouse.pos.0, warehouse.pos.1 * 2);
    Warehouse { layout, pos }
}

fn main() {
    let (warehouse, moves): (Warehouse<P1ObjectKind>, _) = read_input(PATH);
    let warehouse = double_warehouse(warehouse);
    solve(warehouse, &moves);
}
