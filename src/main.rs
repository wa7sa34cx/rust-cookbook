#![allow(dead_code, unused_imports)]

mod ch01_random;
mod ch02_terminal;
// mod ch03_compression;
mod ch04_concurrency;

use ch01_random::distr;

// use ch01_random;

fn main() {
    // ch01_random::random_numbers();
    // ch01_random::within_a_range();
    // ch01_random::rand_uniform();
    // distr::with_distr().unwrap();
    // ch01_random::rand_password();

    let arr = &[1, 25, -4, 10];
    let max = ch04_concurrency::find_max(arr).unwrap();
    println!("{}", max);

}
