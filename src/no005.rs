// URL:https://atcoder.jp/contests/abc095/tasks/arc096_a
// start: 22:43
// start: 23:01

use proconio::input;
use std::cmp;

fn main() {
    input! {
        inputs: [i32;5],
    }

    let a = inputs[0];
    let b = inputs[1];
    let c = inputs[2];

    let x = inputs[3];
    let y = inputs[4];

    let ps = (0..=(2 * cmp::max(x, y)))
        .into_iter()
        .map(|nc| {
            if nc % 2 == 0 {
                let na = cmp::max(x - nc / 2, 0);
                let nb = cmp::max(y - nc / 2, 0);
                return na * a + nb * b + nc * c;
            } else {
                let na = cmp::max(x - (nc - 1) / 2, 0);
                let nb = cmp::max(y - (nc - 1) / 2, 0);
                return na * a + nb * b + nc * c;
            }
        })
        .collect::<Vec<i32>>();
    println!("{:?}", ps.iter().min());
}
