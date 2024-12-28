use crate::maze::{Cell, Maze, Point};

use rustc_hash::FxHashMap as HashMap;

trait CheatHelper {
    fn compute_distances_from_pt(&self, pt: Point) -> HashMap<Point, usize>;

    fn find_all_cheat_scores(
        &self,
        start_scores: &HashMap<Point, usize>,
        end_scores: &HashMap<Point, usize>,
        max_phantom_distance: usize,
    ) -> impl Iterator<Item = (Point, Point, usize)>;
}

impl CheatHelper for Maze {
    fn compute_distances_from_pt(&self, pt: Point) -> HashMap<Point, usize> {
        self.bfs(pt, |to, _, _| matches!(self.get(to), Some(Cell::Open)))
    }

    fn find_all_cheat_scores(
        &self,
        start_scores: &HashMap<Point, usize>,
        end_scores: &HashMap<Point, usize>,
        max_phantom_distance: usize,
    ) -> impl Iterator<Item = (Point, Point, usize)> {
        self.iter()
            .filter(|(_, cell)| matches!(cell, Cell::Open))
            .flat_map(move |(start, _)| {
                let bf_res = self.bfs(start, |to, _, dist| {
                    if dist > max_phantom_distance {
                        return false;
                    }

                    // allow any valid (on-screen) move. this will result in duplicate cheats
                    self.get(to).is_some()
                });
                let ends = bf_res.into_iter();
                let scores = ends
                    .filter(|(end, _)| matches!(self.get(*end), Some(Cell::Open)) && start != *end)
                    .map(move |(end, dist)| {
                        let saving = savings(start_scores, end_scores, start, end, dist);
                        (start, end, saving)
                    });
                scores.collect::<Vec<_>>()
            })
    }
}

pub trait Cheats {
    fn cheat_scores(
        &self,
        start: Point,
        end: Point,
        max_phantom_distance: usize,
    ) -> HashMap<(Point, Point), usize>;
}

impl Cheats for Maze {
    fn cheat_scores(
        &self,
        start: Point,
        end: Point,
        max_phantom_distance: usize,
    ) -> HashMap<(Point, Point), usize> {
        let distances_from_end = self.compute_distances_from_pt(end);
        let distances_from_start = self.compute_distances_from_pt(start);
        self.find_all_cheat_scores(
            &distances_from_start,
            &distances_from_end,
            max_phantom_distance,
        )
        .map(|(a, b, c)| ((a, b), c))
        .collect()
    }
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
