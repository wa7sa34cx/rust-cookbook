#![allow(dead_code, unused_imports)]

mod ch01_random;
mod ch02_terminal;

use ch01_random::distr;

// use ch01_random;

fn main() {
    // ch01_random::random_numbers();
    // ch01_random::within_a_range();
    // ch01_random::rand_uniform();
    // distr::with_distr().unwrap();
    // ch01_random::rand_password();
    ch02_terminal::ansi();
}
