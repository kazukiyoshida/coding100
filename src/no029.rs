// URL:https://atcoder.jp/contests/abc007/tasks/abc007_3
// start:1756
// end:1823

use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp;
use std::collections::HashSet;

struct G {
    h: usize,
    w: usize,
    sx: usize,
    sy: usize,
    gx: usize,
    gy: usize,
    map: Vec<Vec<char>>,
    l: u32,
    q: Vec<(usize, usize)>,
    rsl: Vec<Vec<Option<u32>>>,
}

impl G {
    fn main(&mut self) {
        self.q.push((self.sx - 1, self.sy - 1));
        self.rsl[self.sx - 1][self.sy - 1] = Some(0);

        'outer: while let Some(r) = self.q.pop() {
            self.l += 1;

            let ns = vec![
                (r.0 + 0, r.1 + 1),
                (r.0 + 1, r.1 + 0),
                (r.0 - 1, r.1 + 0),
                (r.0 + 0, r.1 - 1),
            ];

            for n in ns {
                if self.rsl[n.0][n.1] == None && self.map[n.0][n.1] == '.' {
                    self.rsl[n.0][n.1] = Some(self.l);

                    if self.rsl[self.gx - 1][self.gy - 1].is_some() {
                        break 'outer;
                    }
                    self.q.push(n);
                }
            }
        }

        println!("------------------");
        println!(
            "{:?}",
            self.rsl
                .clone()
                .into_iter()
                .flatten()
                .max()
                .unwrap()
                .unwrap()
        );
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
        sx: usize,
        sy: usize,
        gx: usize,
        gy: usize,
        map: [Chars; h],
    }

    G {
        h,
        w,
        map,
        sx,
        sy,
        gx,
        gy,
        l: 0,
        q: vec![],
        rsl: vec![vec![None; w]; h],
    }
    .main();
}
