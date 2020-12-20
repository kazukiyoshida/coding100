// URL:http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1160&lang=jp
// start:
// end:

use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp;
use std::collections::HashSet;

// -----------------------------------------------------------------------------

fn main_old() {
    input! {
        w: usize,
        h: usize,
        map: [[u32;w]; h],
    }

    let mut n = 0;
    let mut memo = vec![vec![None; w]; h];

    for x in (0..h) {
        for y in (0..w) {
            if memo[x][y].is_none() {
                // 海
                if map[x][y] == 0 {
                    memo[x][y] = Some(0);
                // 陸
                } else if map[x][y] == 1 {
                    n += 1;
                    memo[x][y] = Some(n);
                    dfs(&map, &mut memo, n, (x, y), (h, w));
                }
            }
        }
    }
    println!("{:?}", memo);
    println!("n = {:?}", n);
}

fn dfs(
    map: &Vec<Vec<u32>>,
    memo: &mut Vec<Vec<Option<u32>>>,
    n: u32,
    r: (usize, usize),
    wh: (usize, usize),
) {
    // 前提
    if map[r.0][r.1] == 0 {
        return;
    }

    let rx = r.0 as isize;
    let ry = r.1 as isize;

    let nexts = vec![
        (rx - 1, ry + 1),
        (rx + 0, ry + 1),
        (rx + 1, ry + 1),
        (rx + 1, ry + 0),
        (rx + 1, ry - 1),
        (rx + 0, ry - 1),
        (rx - 1, ry - 1),
        (rx - 1, ry + 0),
    ];

    for next in nexts {
        if next.0 >= 0 && next.1 >= 0 && next.0 < wh.0 as isize && next.1 < wh.1 as isize {
            let x = next.0 as usize;
            let y = next.1 as usize;
            if memo[x][y].is_none() && map[x][y] == 1 {
                memo[x][y] = Some(n);
                dfs(&map, memo, n, (x, y), wh)
            }
        }
    }
}

// -----------------------------------------------------------------------------

struct G {
    n: u32,
    w: usize,
    h: usize,
    map: Vec<Vec<u32>>,
    memo: Vec<Vec<Option<u32>>>,
}

impl G {
    fn dfs(&mut self, r: (usize, usize)) {
        // 前提
        if self.map[r.0][r.1] == 0 {
            return;
        }

        let rx = r.0 as isize;
        let ry = r.1 as isize;

        let nexts = vec![
            (rx - 1, ry + 1),
            (rx + 0, ry + 1),
            (rx + 1, ry + 1),
            (rx + 1, ry + 0),
            (rx + 1, ry - 1),
            (rx + 0, ry - 1),
            (rx - 1, ry - 1),
            (rx - 1, ry + 0),
        ];

        for next in nexts {
            if next.0 >= 0 && next.1 >= 0 && next.0 < self.h as isize && next.1 < self.w as isize {
                let x = next.0 as usize;
                let y = next.1 as usize;
                if self.memo[x][y].is_none() && self.map[x][y] == 1 {
                    self.memo[x][y] = Some(self.n);
                    self.dfs((x, y))
                }
            }
        }
    }

    fn main(&mut self) {
        for x in (0..self.h) {
            for y in (0..self.w) {
                if self.memo[x][y].is_none() {
                    // 海
                    if self.map[x][y] == 0 {
                        self.memo[x][y] = Some(0);
                    // 陸
                    } else if self.map[x][y] == 1 {
                        self.n += 1;
                        self.memo[x][y] = Some(self.n);
                        self.dfs((x, y));
                    }
                }
            }
        }
        println!("{:?}", self.memo);
        println!("n = {:?}", self.n);
    }
}

fn main() {
    input! {
        w: usize,
        h: usize,
        map: [[u32;w]; h],
    }

    G {
        n: 0,
        w,
        h,
        map,
        memo: vec![vec![None; w]; h],
    }
    .main();
}
