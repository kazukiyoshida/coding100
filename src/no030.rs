// URL:https://atcoder.jp/contests/joi2011yo/tasks/joi2011yo_e
// start:1830
// end:2000
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp;
use std::collections::HashSet;
use std::fmt::Debug;

struct G {
    h: usize,
    w: usize,
    n: u32,
    map: Vec<Vec<char>>,
    t: u32,
    memo: Vec<Vec<Option<u32>>>,
    q: Vec<Vec<(usize, usize)>>,
    hp: u32,
    count: u32,
}

fn show<T: Debug>(map: &Vec<Vec<Option<T>>>) {
    for xs in map {
        for y in xs {
            if let Some(a) = y {
                print!("{:^4?}, ", a);
            } else {
                print!("{:^4}, ", 0);
            }
        }
        print!("\n");
    }
}

impl G {
    fn main(&mut self) {
        // check start
        let mut r0 = (0, 0);
        for x in 0..self.h {
            for y in 0..self.w {
                if self.map[x][y] == 'S' {
                    r0 = (x, y);
                }
            }
        }
        self.q.push(vec![r0]);
        self.memo[r0.0][r0.1] = Some(0);

        'outer: while let Some(rs) = self.q.pop() {
            self.t += 1;

            let mut tmp = vec![];

            println!("t = {:?} -----------", self.t);
            show(&self.memo);

            'middle: for r in rs {
                let ns = vec![
                    (r.0 as isize + 0, r.1 as isize + 1),
                    (r.0 as isize + 1, r.1 as isize + 0),
                    (r.0 as isize + 0, r.1 as isize - 1),
                    (r.0 as isize - 1, r.1 as isize + 0),
                ];

                for n in ns {
                    if 0 <= n.0 && n.0 < self.h as isize && 0 <= n.1 && n.1 < self.w as isize {
                        let x = n.0 as usize;
                        let y = n.1 as usize;
                        if self.memo[x][y].is_none() {
                            if self.map[x][y] == 'X' {
                                continue;
                            } else if self.map[x][y] == '.' || self.map[x][y] == 'S' {
                                self.memo[x][y] = Some(self.t);
                                tmp.push((x, y));
                                continue;
                            } else {
                                let cheese = self.map[x][y].to_digit(10).unwrap();
                                if cheese <= self.hp {
                                    println!("!!!! cheese !! {:?}", cheese);
                                    self.hp += 1;
                                    self.map[x][y] = '.';
                                    self.memo = vec![vec![None; self.w]; self.h];
                                    self.memo[x][y] = Some(self.t);
                                    self.count += 1;

                                    if self.count == self.n {
                                        break 'outer;
                                    }

                                    self.q = vec![];
                                    tmp = vec![];
                                    tmp.push((x, y));
                                    break 'middle;
                                } else {
                                    self.memo[x][y] = Some(self.t);
                                    tmp.push((x, y));
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
            self.q.push(tmp);
        }
        println!(">>>>>>>>>>>>>>>>>");
        println!(
            "{:?}",
            self.memo
                .clone()
                .into_iter()
                .flatten()
                .max()
                .unwrap()
                .unwrap()
        );
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
        n: u32,
        map: [Chars; h],
    }

    G {
        h,
        w,
        n,
        map: map.clone(),
        memo: vec![vec![None; w]; h],
        q: vec![],
        t: 0,
        hp: 1,
        count: 0,
    }
    .main();
}
