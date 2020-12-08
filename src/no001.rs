// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_7_B&lang=ja

fn a(n: i64, t: i64) -> Vec<[i64; 3]> {
    let n_ = n + 1;
    let mut s = vec![];
    for x in 1..n_ {
        for y in 1..n_ {
            for z in 1..n_ {
                if (x < y && y < z) && (x + y + z == t) {
                    s.push([x, y, z]);
                }
            }
        }
    }
    s
}

fn main() {
    println!("{:?}", a(5, 9).len());
    println!("{:?}", a(0, 0).len());
}
