// URL:http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_10_A&lang=ja
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
    }

    let mut dp = vec![0; n];
    dp[0] = 1;
    dp[1] = 1;

    for ni in 0..n {
        dp[ni] = if ni >= 2 { dp[ni - 1] + dp[ni - 2] } else { 1 }
    }

    println!("{:?}", dp);
}
