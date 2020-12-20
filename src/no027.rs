// URL:https://atcoder.jp/contests/joi2009yo/tasks/joi2009yo_d
// start:0821
// end:

use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp;
use std::collections::HashSet;

fn main() {
    input! {
        m: usize,
        n: usize,
        map: [[u32; m]; n],
    }

    G {
        m,
        n,
        map,
        r0: (0, 0),
        l: 0,
        result: vec![],
    }
    .main()
}

// G と input! は近くにあった方が見やすい
struct G {
    m: usize, // y
    n: usize, // x
    map: Vec<Vec<u32>>,
    r0: (usize, usize), // (x,y)
    l: u32,
    result: Vec<u32>,
}

impl G {
    fn dfs(&mut self, r: (usize, usize), memo: Vec<Vec<Option<u32>>>, l: u32) {
        // validation
        if self.map[r.0][r.1] == 0 {
            return;
        }

        // 四則演算の時だけ isize にキャストする
        let ns = vec![
            (r.0 as isize + 0, r.1 as isize + 1),
            (r.0 as isize + 1, r.1 as isize + 0),
            (r.0 as isize + 0, r.1 as isize - 1),
            (r.0 as isize - 1, r.1 as isize + 0),
        ];

        for n in ns {
            // 配列の区間に入っていると判定できたら usize にキャストしなおす
            if 0 <= n.0 && n.0 < self.n as isize && 0 <= n.1 && n.1 < self.m as isize {
                let x = n.0 as usize;
                let y = n.1 as usize;

                if self.map[x][y] == 1 && memo[x][y] == None {
                    let mut memo_ = memo.clone();
                    memo_[x][y] = Some(l + 1);
                    self.dfs((x, y), memo_, l + 1);
                }
            }
        }

        let l = memo
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

                self.r0 = (x, y);
                let mut memo = vec![vec![None; self.m]; self.n];
                memo[self.r0.0][self.r0.1] = Some(0);
                self.dfs(self.r0, memo, 0);
            }
        }
        println!("result = {:?}", self.result.clone().into_iter().max());
    }
}
