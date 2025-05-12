
fn main() {

    let number: f64 = std::env::args()
        .skip(1)
        .collect::<String>()
        .parse()
        .expect("first argument should be a number");

    println!("{number}");
}

