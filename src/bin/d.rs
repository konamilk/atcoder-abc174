use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};

fn main() {
    input! {
        n: i32,
        c: Chars
    }

    let n_red = c.iter().filter(|&x| x == &'R').count();

    // dbg!(n_red);

    let sliced = &c[0..n_red];

    let n_red_in_sliced = sliced.iter().filter(|&x| x == &'R').count();

    // dbg!(n_red_in_sliced);

    println!("{}", n_red - n_red_in_sliced)
}
