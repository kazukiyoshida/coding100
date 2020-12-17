// URL:http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1160&lang=jp
// start:
// end:

use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp;
use std::collections::HashSet;

fn main() {
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
