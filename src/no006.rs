// URL:https://atcoder.jp/contests/sumitrust2019/tasks/sumitb2019_d
// start : 00:22
// start : 01:04

use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        s: Chars
    }
    let pins: HashSet<String> = (0..n)
        .combinations(n - 3)
        .map(|is| {
            let mut s_ = s.to_vec();
            for i in is.iter().rev() {
                s_.remove(*i);
            }
            let pin: String = s_.iter().collect();
            pin
        })
        .collect();
    println!("{:?}", pins);
    println!("{:?}", pins.len());
}
