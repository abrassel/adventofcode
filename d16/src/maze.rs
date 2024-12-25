use pheap::PairingHeap;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

use crate::point::{Cell, Dir, Point};

const TURN_COST: u32 = 1000;
const STEP_COST: u32 = 1;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct State {
    loc: Point,
    dir: Dir,
}

impl State {
    pub fn neighbors(self) -> impl Iterator<Item = (Self, u32)> {
        let dir = self.dir;
        [
            self.with_new_dir(dir.turn_ccw()),
            self.with_new_dir(dir.turn_cw()),
        ]
        .into_iter()
        .chain({
            let new_pt = dir.step(self.loc);
            new_pt.map(|loc| {
                let new_state = Self { loc, ..self };
                (new_state, STEP_COST)
            })
        })
    }

    fn with_new_dir(self, dir: Dir) -> (Self, u32) {
        (Self { dir, ..self }, TURN_COST)
    }
}

pub struct Maze {
    layout: Vec<Vec<Cell>>,
    start: State,
    end: Point,
}

impl Maze {
    pub fn djikstra_all_paths(&self) -> usize {
        let mut to_visit = PairingHeap::new();
        let mut cost = HashMap::default();
        let mut prevs: HashMap<_, Vec<State>> = HashMap::default();
        to_visit.insert((self.start, None), 0);
        while let Some(((cur, prev), dist)) = to_visit.delete_min() {
            // terminate if we reach an already-seen location
            // and our cost is greater
            // if cost is the same, then add it to the list of viable previous
            match cost.entry(cur) {
                std::collections::hash_map::Entry::Occupied(occupied_entry) => {
                    if *occupied_entry.get() == dist {
                        if let Some(prev) = prev {
                            prevs.get_mut(&cur).unwrap().push(prev);
                        }
                    }
                    continue;
                }
                std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(dist);
                    if let Some(prev) = prev {
                        prevs.entry(cur).or_default().push(prev);
                    }
                }
            };

            // end of the current search
            if cur.loc == self.end {
                continue;
            }

            for (nbor, edge_weight) in self.neighbors(cur) {
                to_visit.insert((nbor, Some(cur)), dist + edge_weight);
            }
        }

        // reconstruct all paths
        let unique_locs = self.unique_path_locs(cost, prevs);
        self.print_maze_state(unique_locs.clone());

        unique_locs.len()
    }

    fn unique_path_locs(
        &self,
        cost: HashMap<State, u32>,
        prevs: HashMap<State, Vec<State>>,
    ) -> HashSet<Point> {
        let mut unique_locs = HashSet::default();
        // find minimum cost path to self.end
        let min_end_state_cost = cost
            .iter()
            .filter_map(|(k, v)| (k.loc == self.end).then_some(v))
            .min()
            .unwrap();
        let mut visit_stack = vec![
            cost.iter()
                .find_map(|(k, v)| (v == min_end_state_cost && k.loc == self.end).then_some(*k))
                .unwrap(),
        ];
        while let Some(cur) = visit_stack.pop() {
            if cur != self.start {
                debug_assert!(prevs.contains_key(&cur));
            }
            unique_locs.insert(cur.loc);
            for prev in prevs.get(&cur).into_iter().flatten() {
                visit_stack.push(*prev);
            }
        }
        unique_locs
    }

    pub fn djikstra_single_path(&self) -> Option<u32> {
        let mut to_visit = PairingHeap::new();
        let mut seen = HashSet::default();
        to_visit.insert((self.start, vec![]), 0);
        while let Some(((cur, path), dist)) = to_visit.delete_min() {
            if cur.loc == self.end {
                return Some(dist);
            }

            if seen.contains(&cur) {
                continue;
            }
            seen.insert(cur);
            let mut new_path = path.clone();
            new_path.push(cur.loc);

            for (nbor, edge_weight) in self.neighbors(cur) {
                to_visit.insert((nbor, new_path.clone()), dist + edge_weight);
            }
        }

        None
    }

    fn neighbors(&self, n: State) -> impl Iterator<Item = (State, u32)> {
        n.neighbors()
            .filter(|(State { loc, .. }, _)| self.get(loc) == Some(&Cell::Open))
    }

    fn get(&self, pt: &Point) -> Option<&Cell> {
        self.layout.get(pt.0)?.get(pt.1)
    }

    pub fn print_maze_state(&self, valid_spots: HashSet<Point>) {
        for (ridx, row) in self.layout.iter().enumerate() {
            for (cidx, cell) in row.iter().enumerate() {
                if self.end == Point(ridx, cidx) {
                    print!("E");
                } else if valid_spots.contains(&Point(ridx, cidx)) {
                    print!("O");
                } else {
                    print!("{}", cell);
                }
            }
            println!();
        }
    }
}

impl FromIterator<String> for Maze {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut start = None;
        let mut end = None;
        let layout = iter
            .into_iter()
            .enumerate()
            .map(|(ridx, row)| {
                row.char_indices()
                    .map(|(cidx, cell)| {
                        let pt = Point(ridx, cidx);
                        if cell == 'S' {
                            start = Some(State {
                                loc: pt,
                                dir: Dir::East,
                            });
                        } else if cell == 'E' {
                            end = Some(pt);
                        }
                        Cell::from(cell)
                    })
                    .collect()
            })
            .collect();
        Self {
            layout,
            start: start.unwrap(),
            end: end.unwrap(),
        }
    }
}
