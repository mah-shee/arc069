#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        s: usize,
        c: usize,
    }
    if s * 2 >= c {
        println!("{}", c / 2);
    } else {
        let rest = s + (c - s * 2) / 4;
        println!("{}", rest);
    }
}
