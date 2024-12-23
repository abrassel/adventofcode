use d13::{NumSize, Problem, read_input, vec2d::Vec2D};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub fn try_div(top: NumSize, bottom: NumSize) -> Option<NumSize> {
    (top % bottom == 0).then_some(top / bottom)
}

pub fn sub_with_sign(lhs: NumSize, rhs: NumSize) -> (bool, NumSize) {
    if lhs < rhs {
        (true, rhs - lhs)
    } else {
        (false, lhs - rhs)
    }
}

fn solve_problem(problem: Problem) -> Option<Vec2D> {
    let Problem { a, b, v } = problem;
    let (sign1, top) = sub_with_sign(a.1 * v.0, a.0 * v.1);
    let (sign2, bottom) = sub_with_sign(a.1 * b.0, a.0 * b.1);
    if sign1 != sign2 {
        return None;
    }

    let y = try_div(top, bottom)?;
    if v.0 < b.0 * y {
        return None;
    }
    let x = try_div(v.0 - b.0 * y, a.0)?;
    Some(Vec2D(x, y))
}

pub fn main() {
    let problems = read_input();
    let cost: NumSize = problems
        .into_par_iter()
        .filter_map(solve_problem)
        .map(|sol| sol.cost())
        .sum();
    println!("The answer is {cost}");
}
