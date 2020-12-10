use itertools::Itertools;
use proconio::input;
use std::cmp::max;

// URL: https://atcoder.jp/contests/pakencamp-2019-day3/tasks/pakencamp_2019_day3_c
// start: 21:39
// start: 21:52

fn main_() {
    input! {
        n: usize,
        m: usize,
        mat: [[u32; m]; n],
    }

    let mut ps = vec![];
    for i in 0..m {
        for j in 0..m {
            if i < j {
                // 曲のペア( i, j ) に対して最大値を計算する
                let mut p = 0;
                for k in 0..n {
                    p += if mat[k][i] > mat[k][j] {
                        mat[k][i]
                    } else {
                        mat[k][j]
                    }
                }
                ps.push(p);
            }
        }
    }

    // now you can use n, m and l as variable.
    println!("{} {} {:?}", n, m, mat);
    println!("{:?}", ps);

    println!("{:?}", ps.iter().max());
}

fn main() {
    input! {
        n: usize,
        m: usize,
        mat: [[u32; m]; n],
    }
    let mut ps = vec![];
    for pair in (0..m).combinations(2) {
        let p = (0..n).fold(0, |acc, k| acc + max(mat[k][pair[0]], mat[k][pair[1]]));
        ps.push(p);
    }
    println!("{:?}", ps.iter().max());
}
