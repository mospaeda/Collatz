

fn main() {
    use std::cmp::max;
    use num_bigint::{ToBigInt};
    use num_bigint::BigInt;
    let one = 0.to_bigint().unwrap();

    let nb_max:u128=1000000;
    let mut max_height_total=0.to_bigint().unwrap();
    
    

// usage of for loop
    for iii in 1..nb_max {
        // A counter variable
        let mut n:BigInt; 
        let mut counter:u128 =0;
        let mut max_height:BigInt;

        max_height =0.to_bigint().unwrap();
        n =iii.to_bigint().unwrap();

        while  n > 1.to_bigint().unwrap() {
            if &n%2 == 0.to_bigint().unwrap() {
                n=n/2.to_bigint().unwrap();
            }
            else {
                n=3.to_bigint().unwrap()*n+1.to_bigint().unwrap();
            }
            counter += 1;
            if  n > max_height {
                max_height = n.clone();
            }
            max_height=max(max_height.clone(),n.clone());
        }
            if max_height > max_height_total {
            println!("{}  -  count: {}  -  maxH: {}", iii, counter, max_height.clone()/2 );
            max_height_total = max_height.clone();
            }   
    }


}




