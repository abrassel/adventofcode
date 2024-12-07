use derive_more::derive::From;
use itertools::Itertools;

use crate::maze::Pos;

#[derive(From, Debug)]
pub struct Path(Vec<Pos>);

impl Path {
    pub fn iter(&self) -> Vec<(usize, usize)> {
        self.0
            .iter()
            .tuple_windows()
            .map(|(&start, &end)| {
                let (rdir, cdir) = start.2.dir();
                let mut cur = start;
                let mut steps = Vec::new();
                while cur.loc() != end.loc() {
                    if cur.loc() != self.0[0].loc() {
                        steps.push(cur.loc());
                    }
                    cur.0 = (cur.0 as i64 + rdir) as usize;
                    cur.1 = (cur.1 as i64 + cdir) as usize;
                }
                steps
            })
            .flatten()
            .collect_vec()
    }
}

pub trait MapEither<A, B> {
    fn map_either(self, map: impl FnMut(A) -> B) -> Result<B, B>;
}

impl<A, B> MapEither<A, B> for Result<A, A> {
    fn map_either(self, mut map: impl FnMut(A) -> B) -> Result<B, B> {
        match self {
            Ok(a) => Ok(map(a)),
            Err(a) => Err(map(a)),
        }
    }
}
