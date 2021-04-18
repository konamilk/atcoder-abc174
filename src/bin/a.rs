use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
use num_integer::Integer;

fn main() {
    input! {
        x: i32
    }

    if x >= 30 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}
