// URL:https://qiita.com/drken/items/a5e6fe22863b7992efdb#%E5%95%8F%E9%A1%8C-3%E9%83%A8%E5%88%86%E5%92%8C%E5%95%8F%E9%A1%8C
// start:
// end:

use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp;
use std::collections::HashSet;
use std::fmt::Debug;

fn main() {
    input! {
        n: usize,
        A: u32,
        a: [u32; n],
    }

    let mut dp = vec![vec![false; A as usize + 1]; n + 1];
    dp[0][0] = true;

    for ni in 0..n {
        for aj in 0..=A {
            if (dp[ni][aj as usize]) || (aj >= a[ni] && dp[ni][(aj - a[ni]) as usize]) {
                dp[ni + 1][aj as usize] = true;
            } else {
                dp[ni + 1][aj as usize] = false;
            }
        }
    }
    show(&dp);
}

fn show<T: Debug>(map: &Vec<Vec<T>>) {
    println!("------------");
    for xs in map {
        for y in xs {
            print!("{:^3?}, ", y);
        }
        print!("\n");
    }
}
