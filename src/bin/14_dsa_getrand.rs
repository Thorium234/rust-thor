/*use rand::{thread_rng, Rng};

fn main(){
let mut rng = thread_rng();

// Exclusive range
let n: u32 = rng.gen_range(0..10);
println!("{}", n);
let m: f64 = rng.gen_range(-40.0..1.3e5);
println!("{}", m);

// Inclusive range
let n: u32 = rng.gen_range(0..=10);
println!("{}", n);

}

use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();
    // Example: generate a number from 1 to 11
    let n: u32 = rng.gen_range(1..12);
    println!("Random number: {}", n);
}*/

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    // Range [1, 100) -> 1 to 99
    let n1: u32 = rng.gen_range(1..100);

    // Range [1, 100] -> 1 to 100
    let n2: u32 = rng.gen_range(1..=100);

    println!("Random numbers: {}, {}", n1, n2);
}


