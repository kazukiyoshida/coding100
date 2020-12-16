// URL:http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_4_B&lang=ja
// start:
// end:

mod tools;

use crate::tools::binary_search::BinarySearch;
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        ss: [u32; n],
        q: usize,
        ts: [u32; q],
    }
    let l = ts
        .into_iter()
        .filter(|t| ss.contains(&t))
        .collect::<Vec<u32>>()
        .len();
    println!("{:?}", l);
}
