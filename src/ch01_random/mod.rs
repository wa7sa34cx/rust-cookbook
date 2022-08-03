pub mod distr;

use rand::distributions::Alphanumeric;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;

// Generate random numbers
pub fn random_numbers() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();

    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
}

// Generate random numbers within a range
pub fn within_a_range() {
    let mut rng = rand::thread_rng();

    println!("Integer: {}", rng.gen_range(0..10));
    println!("Float: {}", rng.gen_range(0.0..10.0));
}

// Uniform distribution
pub fn rand_uniform() {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);

    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }
}

pub fn rand_password() {
    let rand_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    println!("{}", rand_string);
}
