use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let random_number: u32 = rng.gen_range(1..101);
    println!("Random number: {random_number}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_number() {
        let mut rng = rand::thread_rng();
        let _: u32 = rng.gen_range(1..101);
    }
}
