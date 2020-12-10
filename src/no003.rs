// URL: https://atcoder.jp/contests/abc122/tasks/abc122_b
// start : 23:04
// end : 23:29

fn calc(s: &str) -> i64 {
    let mut stack = vec![];
    let mut count = 0;
    let mut flg = false;

    for i in 0..(s.len()) {
        if s.chars().nth(i) == Some('A')
            || s.chars().nth(i) == Some('T')
            || s.chars().nth(i) == Some('C')
            || s.chars().nth(i) == Some('G')
        {
            flg = true;
            count += 1;

            // 終端
        } else if flg {
            flg = false;
            stack.push(count);
        }
    }

    *stack.iter().max().unwrap_or_else(|| &0)
}

fn main() {
    println!("{:?}", calc("ATCODER"));
    println!("{:?}", calc("HATAGAYA"));
    println!("{:?}", calc("SHINJUKU"));
}
