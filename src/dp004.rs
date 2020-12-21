// URL:https://qiita.com/drken/items/a5e6fe22863b7992efdb#%E5%95%8F%E9%A1%8C-4%E9%83%A8%E5%88%86%E5%92%8C%E6%95%B0%E3%81%88%E4%B8%8A%E3%81%92%E5%95%8F%E9%A1%8C
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
        A: usize,
        a: [usize; n],
    }

    let mut dp = vec![vec![0; A + 1]; n + 1];
    dp[0][0] = 1;

    for ni in 0..n {
        for Aj in 0..=A {
            dp[ni + 1][Aj] = if Aj >= a[ni] {
                dp[ni][Aj] + dp[ni][(Aj - a[ni])]
            } else {
                dp[ni][Aj]
            };
        }
    }

    println!("{:?}", dp);
}
