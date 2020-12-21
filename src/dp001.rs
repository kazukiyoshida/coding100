// URL:https://qiita.com/drken/items/a5e6fe22863b7992efdb#%E5%95%8F%E9%A1%8C-1%E6%9C%80%E5%A4%A7%E5%92%8C%E5%95%8F%E9%A1%8C
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
        a: [i32; n],
    }

    let mut dp = vec![];
    dp.push(0);

    for i in 0..n {
        let k = if a[i] >= 0 { a[i] } else { 0 };
        dp.push(dp[i] + k);
    }

    println!("{:?}", dp);
}
