
fn main() {
    let args: Vec<String> = std::env::args().collect();

    let length = args
        .get(1)
        .expect("A line length should be given as argument 1.")
        .parse()
        .expect("Argument 1 should be a number.");

    let character = args
        .get(2)
        .expect("A line-character should be given as argument 2.")
        .chars()
        .next()
        .expect("Argument 2 should not be empty.");

    for _ in 0..length { print!("{character}"); }
    println!();
}

