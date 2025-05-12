
use std::io::BufRead;


fn print_binary(text: &str) {
    for character in text.chars() {
        print!("{:b}", character as u8);
    }
}


fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        for word in &args[1..] {
            print_binary(&word);
        }
    }
    else {
        for line in std::io::stdin().lock().lines() {
            let line = line.expect("Should have been able to read line from stdin.");
            print_binary(&line);
        }
    }

    println!();
}

