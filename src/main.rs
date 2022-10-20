fn main() {
    println!("Hello, world!");


use num_bigint::{ToBigInt, RandBigInt};

let mut rng = rand::thread_rng();
let a = rng.gen_bigint(1000);

let low = -10000.to_bigint().unwrap();
let high = 10000.to_bigint().unwrap();
let b = rng.gen_bigint_range(&low, &high);

// Probably an even larger number.
println!("{}", a * b);
}

