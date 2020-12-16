// URL:https://www.ioi-jp.org/joi/2007/2008-ho-prob_and_sol/2008-ho.pdf#page=6
// start:0621
// end:0749

mod tools;

use crate::tools::binary_search::BinarySearch;
use itertools::Itertools;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: u32,
        mut ps: [u32; n],
    }

    // 0 を追加
    ps.push(0);

    let ps = ps
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    let i = (0..m).collect::<Vec<u32>>().upper_bound(|&k| {
        // 与えられた k に対して、 ps を最大 4 つまで合計することで k <= sum < m とすることができるか？
        for case in ps.iter().combinations_with_replacement(4) {
            let s = case.into_iter().sum::<u32>();
            if k <= s && s < m {
                return true;
            }
        }
        return false;
    });
    println!("{:?}", ps);
    println!("{:?}", i);
}
