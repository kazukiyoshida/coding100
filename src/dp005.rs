// URL:https://qiita.com/drken/items/a5e6fe22863b7992efdb#%E5%95%8F%E9%A1%8C-5%E6%9C%80%E5%B0%8F%E5%80%8B%E6%95%B0%E9%83%A8%E5%88%86%E5%92%8C%E5%95%8F%E9%A1%8C
// // start:
// end:

use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        A: usize,
        a: [usize; n],
    }

    // 十分大きい値を作成する
    const INF: usize = 1 << 29;
    println!("{:?}", INF);

    // 一旦全て INF にする.
    let mut dp = vec![vec![INF; A + 1]; n + 1];
    dp[0][0] = 0;

    for ni in 0..n {
        for Aj in 0..=A {
            dp[ni + 1][Aj] = if Aj >= a[ni] {
                cmp::min(dp[ni][Aj - a[ni]] + 1, dp[ni][Aj])
            } else {
                dp[ni][Aj]
            };
        }
    }

    println!("{:?}", dp);
}
