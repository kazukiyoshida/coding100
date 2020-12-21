// URL:http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_A&lang=ja
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
        m: usize,
        c: [usize; m],
    }

    const INF: usize = 1 << 29;

    let mut dp = vec![vec![INF; n + 1]; m + 1];
    dp[0][0] = 0;

    for mi in 0..m {
        for nj in 0..=n {
            if nj >= c[mi] {
                let a = nj / c[mi];
                dp[mi + 1][nj] =
                    if let Some(min) = (0..=a).map(|k| dp[mi][nj - c[mi] * k] + k).min() {
                        min
                    } else {
                        INF
                    }
            } else {
                dp[mi + 1][nj] = dp[mi][nj];
            }
        }
    }

    println!("{:?}", dp);
}
