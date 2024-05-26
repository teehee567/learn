#![allow(unused)]

mod chunked;
mod naive;
use chunked::*;
use naive::*;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    {
        Naive::start("measurements.txt");
    }

    let elapsed = now.elapsed();
    println!("elapsed: {:.2?}", elapsed);
}
