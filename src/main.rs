

fn main() {

    use num_bigint::{ToBigInt};
    use num_bigint::BigInt;

        let one = 1.to_bigint().unwrap();
let nb_max:u128=10;

println!("{}", one);

// usage of for loop
    for iii in 1..nb_max {
        // A counter variable
        let mut n:BigInt; 
        let mut counter:u128 =0;

        n =iii.to_bigint().unwrap();
        // .to_bigint();
 

    // Loop while `n` is less than 101
    while  n > 1.to_bigint().unwrap() {
        if &n%2 == 0.to_bigint().unwrap() {
            n=n/2.to_bigint().unwrap();
        }
        else {
            n=3.to_bigint().unwrap()*n+1.to_bigint().unwrap()
        }
        // Increment counter
        counter += 1;

    }
    println!("number: {}  -  count: {}", iii, counter );
}
}

