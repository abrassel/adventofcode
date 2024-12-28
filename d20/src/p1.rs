use rustc_hash::FxHashMap as HashMap;

use d20::{
    Setup,
    maze::{Cell, Maze, Point},
    read_input,
};

const PATH: &str = "d20/input/input.txt";

fn compute_distances_from_pt(maze: &Maze, pt: Point) -> HashMap<Point, usize> {
    maze.bfs(pt, |to, _, _| matches!(maze.get(to), Some(Cell::Open)))
}

fn savings(
    start_scores: &HashMap<Point, usize>,
    end_scores: &HashMap<Point, usize>,
    start: Point,
    end: Point,
    dist: usize,
) -> usize {
    let new_score = start_scores[&start] + end_scores[&end] + dist;
    let old_score = start_scores[&start] + end_scores[&start];
    old_score.saturating_sub(new_score)
}

fn find_all_cheat_scores(
    maze: &Maze,
    start_scores: &HashMap<Point, usize>,
    end_scores: &HashMap<Point, usize>,
) -> impl Iterator<Item = (Point, Point, usize)> {
    maze.iter()
        .filter(|(_, cell)| matches!(cell, Cell::Open))
        .flat_map(move |(start, _)| {
            let bf_res = maze.bfs(start, |to, from, dist| {
                if dist > 2 {
                    return false;
                }

                // choose which "cheating" moves are allowed
                match (maze.get(from).unwrap(), maze.get(to)) {
                    // disallow: moving off-screen | moving from an open cell to another open cell
                    (_, None) | (Cell::Open, Some(Cell::Open)) => false,
                    // allow: moving from a closed cell to an open cell | moving from an open cell to a closed cell | moving between closed cells
                    (Cell::Closed, Some(Cell::Open))
                    | (Cell::Open, Some(Cell::Closed))
                    | (Cell::Closed, Some(Cell::Closed)) => true,
                }
            });
            let ends = bf_res.into_iter();
            let scores = ends
                .filter(|(end, _)| matches!(maze.get(*end), Some(Cell::Open)) && start != *end)
                .map(move |(end, dist)| {
                    let saving = savings(start_scores, end_scores, start, end, dist);
                    (start, end, saving)
                });
            scores.collect::<Vec<_>>()
        })
}

fn main() {
    let Setup { maze, start, end } = read_input(PATH);
    let distances_from_end = compute_distances_from_pt(&maze, end);
    let distances_from_start = compute_distances_from_pt(&maze, start);
    let cheat_scores = find_all_cheat_scores(&maze, &distances_from_start, &distances_from_end);
    let score = cheat_scores.filter(|(_, _, score)| *score >= 100).count();
    println!("The number of such cheats is {}", score);
}
