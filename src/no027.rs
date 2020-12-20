// URL:https://atcoder.jp/contests/joi2009yo/tasks/joi2009yo_d
// start:0821
// end:

use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp;
use std::collections::HashSet;

struct G {
    m: isize, // y
    n: isize, // x
    map: Vec<Vec<u32>>,
    start: (usize, usize), // (x,y)
    len: u32,
    result: Vec<u32>,
}

impl G {
    fn dfs(&mut self, r: (usize, usize), memo: Vec<Vec<Option<u32>>>, len: u32) {
        // validation
        if self.map[r.0][r.1] == 0 {
            return;
        }

        let ns = vec![
            (r.0 as isize + 0, r.1 as isize + 1),
            (r.0 as isize + 1, r.1 as isize + 0),
            (r.0 as isize + 0, r.1 as isize - 1),
            (r.0 as isize - 1, r.1 as isize + 0),
        ];

        for n in ns {
            if 0 <= n.0 && n.0 < self.n && 0 <= n.1 && n.1 < self.m {
                let x = n.0 as usize;
                let y = n.1 as usize;

                if self.map[x][y] == 1 && memo[x][y] == None {
                    let mut cloned_memo = memo.clone();
                    cloned_memo[x][y] = Some(len + 1);
                    self.dfs((x, y), cloned_memo, len + 1);
                }
            }
        }

        let l = memo
            .clone()
            .into_iter()
            .flatten()
            .filter(|m| m.is_some())
            .map(|m| m.unwrap())
            .max()
            .unwrap();
        self.result.push(l);
    }

    fn main(&mut self) {
        for x in 0..self.n {
            for y in 0..self.m {
                println!("------------");
                println!("{:?}", (x, y));

                self.start = (x as usize, y as usize);
                let mut memo = vec![vec![None; self.m as usize]; self.n as usize];
                memo[self.start.0][self.start.1] = Some(0);
                self.dfs(self.start, memo, 0);
            }
        }
        println!(
            "######## result = {:?}",
            self.result.clone().into_iter().max()
        );
    }
}

fn main() {
    input! {
        m: isize,
        n: isize,
        map: [[u32; m]; n],
    }

    G {
        m,
        n,
        map,
        start: (0, 0),
        len: 0,
        result: vec![],
    }
    .main()
}
