// URL:https://atcoder.jp/contests/abc023/tasks/abc023_d
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
        mut ps: [(u32, u32); n],
    }
    let max_t = n as u32 - 1;
    let max_speed = ps.clone().into_iter().map(|x| x.1).max().unwrap();
    let init_max_height = ps.clone().into_iter().map(|x| x.0).max().unwrap();
    let max_height = max_t * max_speed + init_max_height;

    let init_min_height = ps.clone().into_iter().map(|x| x.0).min().unwrap();

    // 「割り切れない」の下限が得られる. 従って、point は得られた値よりも 1つ下.
    let point = (0..=max_height).collect::<Vec<u32>>().lower_bound(|&h| {
        // 高度 h に対して、それまでに全て割り切れるか？
        if ps
            .clone()
            .into_iter()
            .filter(|x| x.0 > h)
            .collect::<Vec<(u32, u32)>>()
            .len()
            > 0
        {
            return false;
        }
        ps.clone()
            .into_iter()
            .map(|x| (h - x.0) as f32 / (x.1 as f32))
            .filter(|s| *s > (n - 1) as f32)
            .collect::<Vec<f32>>()
            .len()
            > 0
    });
    println!("--- point = {:?}", point);
}
