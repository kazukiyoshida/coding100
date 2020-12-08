// https://atcoder.jp/contests/abc106/tasks/abc106_b
//
// 9:38
// 1, 3, 5, 7, 15, 21, 35, 105

fn rule(a: i64) -> bool {
    if a % 2 == 0 {
        return false;
    }
    let mut c = 0;
    for x in 1..(a + 1) {
        if a % x == 0 {
            c += 1;
        }
    }
    c == 8
}

fn rule_(a: i64) -> bool {
    if a % 2 == 0 {
        return false;
    }
    (1..=a).filter(|x| a % x == 0).collect::<Vec<i64>>().len() == 8usize
}

fn c(N: i64) -> i64 {
    let mut r: i64 = 0;
    for k in 1..(N + 1) {
        if rule_(k) {
            r += 1
        }
    }
    r
}

fn c_(N: i64) -> usize {
    (1..=N).filter(|k| rule(*k)).collect::<Vec<i64>>().len()
}

fn main() {
    println!("{}", c(7));
    println!("{}", c(105));
    println!("{}", c_(7));
    println!("{}", c_(105));
}
