// URL:https://qiita.com/drken/items/a5e6fe22863b7992efdb#%E5%95%8F%E9%A1%8C-2%E3%83%8A%E3%83%83%E3%83%97%E3%82%B5%E3%83%83%E3%82%AF%E5%95%8F%E9%A1%8C
// start:
// end:
//
// input
// 6
// 9
// 2 3
// 1 2
// 3 6
// 2 1
// 1 3
// 5 85

use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp;
use std::collections::HashSet;
use std::fmt::Debug;

fn main() {
    input! {
        n: usize,
        w: u32,
        pair: [(u32, u32); n], // weight, value
    }

    // テーブルは先に作成する
    // タテ：0 .. =n
    // ヨコ：0 .. =w
    let mut dp = vec![vec![0u32; 1 + w as usize]; n + 1];

    for i in 0..n {
        for m in 0..=w {
            if pair[i].0 <= m {
                let l = dp[i][(m - pair[i].0) as usize] + pair[i].1;
                let r = dp[i][m as usize];
                dp[i + 1][m as usize] = cmp::max(l, r);
            } else {
                dp[i + 1][m as usize] = dp[i][m as usize]
            }
        }
        show(&dp);
    }
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
