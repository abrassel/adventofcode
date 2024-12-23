use d13::{NumSize, Problem, read_input, vec2d::Vec2D};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

trait SolveP1 {
    fn is_valid(&self, y: NumSize) -> bool;

    fn solve(&self) -> Option<Vec2D>;
}

impl SolveP1 for Problem {
    fn is_valid(&self, y: NumSize) -> bool {
        (self.v - self.b * y).div_rem(self.a).valid()
    }

    fn solve(&self) -> Option<Vec2D> {
        // assume that a is more valuable, so maximize b
        let mut maxy = None;
        let mut cury = 0;
        while (self.b * cury).lte(self.v) {
            if self.is_valid(cury) {
                maxy = Some(std::cmp::max(maxy.unwrap_or_default(), cury));
            }
            cury += 1;
        }
        let maxy = maxy?;
        let xs = (self.v - self.b * maxy) / self.a;
        debug_assert_eq!(xs.0, xs.1, "{:?} {}", self, maxy);
        Some(Vec2D(xs.0, maxy))
    }
}

pub fn main() {
    let problems = read_input();
    let cost: NumSize = problems
        .into_par_iter()
        .filter_map(|problem| problem.solve())
        .map(|sol| sol.cost())
        .sum();
    println!("The answer is {cost}");
}
