// URL:http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_5_A&lang=ja#
// start:
// end:

use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp;
use std::collections::HashSet;

// bit 全探索の基本コード
fn sample() {
    let v = "ABCD".chars().collect::<Vec<char>>();
    println!("{:?}", v);

    for case in 0..(1 << v.len()) {
        print!("{{");

        for n in 0..v.len() {
            if (1 << n) & case == 0 {
                print!("{}", v[n]);
            }
        }

        print!("\n");
    }
}

fn main() {
    input! {
        n: usize,
        A: [i32; n],
        q: usize,
        ms: [i32; q],
    }
    let r: Vec<bool> = ms.iter().map(|m| judge(A.to_vec(), *m)).collect();
    println!("{:?}", r);
}

fn judge(A: Vec<i32>, m: i32) -> bool {
    for case in 0..(1 << A.len()) {
        if (0..A.len())
            .filter(|n| (1 << n) & case == 0)
            // n は index なので、対応する A の要素を取得する必要あり
            .map(|n| A[n])
            .fold(0, |acc, k| acc + k)
            == m
        {
            return true;
        }
    }
    false
}
