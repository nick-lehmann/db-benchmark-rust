#![feature(int_log)]
use std::time::SystemTime;

fn main() {
    let start = SystemTime::now();

    let mut sum: i64 = 0;
    for i in 0..1_000_000 {
        sum += i;
    }
    println!("{}", sum);

    let diff = start.elapsed().unwrap();
    let digits = diff.as_nanos().log10() + 1;

    println!("Took {}ns ({})", diff.as_nanos(), digits);
}
