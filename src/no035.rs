// URL:http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_B&lang=ja
// start:
// end:

use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        w: usize,
        p: [(usize, usize); n], // value, weight
    }

    let mut dp = vec![vec![0; w + 1]; n + 1];
    for ni in 0..n {
        for wj in 0..=w {
            dp[ni + 1][wj] = if p[ni].1 <= wj {
                cmp::max(dp[ni][wj - p[ni].1] + p[ni].0, dp[ni][wj])
            } else {
                dp[ni][wj]
            }
        }
    }

    println!("{:?}", dp);
}
