
use std::io::Read;
use rand::Rng;
use rand::distributions::{Bernoulli, Distribution};


fn get_rand_byte() -> u8 {
    rand::thread_rng().gen_range(0..=u8::MAX)
}


fn main() {
    let args: Vec<String> = std::env::args().collect();

    let amount: f64 = 
        if args.get(1).is_some() {
            args.get(1).unwrap().parse().expect("Argument 1 should be a number")
        }
        else {
            0.2
        };

    let distribution = Bernoulli::new(amount).unwrap();
    for byte in std::io::stdin().lock().bytes() {

        let byte =
            if distribution.sample(&mut rand::thread_rng()) { get_rand_byte() }
            else { byte.unwrap() };

        print!("{}", byte as char);
    }
}

