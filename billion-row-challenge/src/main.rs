#![allow(unused)]

mod chunked;
mod naive;

use chunked::*;
use naive::*;

mod betterhash;
use betterhash::*;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    {
        BetterHash::start("measurements.txt");
    }

    let elapsed = now.elapsed();
    println!("elapsed: {:.2?}", elapsed);
}
