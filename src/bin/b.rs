use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};

fn main() {
    input! {
        n: usize,
        d: i64,
        xy: [(i64, i64); n]
    }

    let mut ans = 0;

    for i in 0..n{
        if xy[i].0 * xy[i].0 + xy[i].1 * xy[i].1 <= d*d {
            ans += 1;
        }
    }

    println!("{}", ans);
}
