
use std::io::Write;


fn postfix_to_factor(postfix: char) -> Option<u64> {
    match postfix {
        's' => Some(1),
        'm' => Some(60),
        'h' => Some(3600),
        _ => None,
    }
}


fn count_down(mut timer: u64) {
    while timer > 0 {
        let hours = timer / 3600;
        let minutes = (timer % 3600) / 60;
        let seconds = timer % 60;

        print!("{:02}:{:02}:{:02}\r", hours, minutes, seconds);
        std::io::stdout()
            .flush()
            .expect("Stdout should be flushable");

        timer -= 1;
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    println!("{:02}:{:02}:{:02}", 0, 0, 0);
}


fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut timer: u64 = args
        .get(1)
        .expect("Time should be specified as argument 1")
        .parse()
        .expect("Argument 1 should be a number");

    if let Some(postfix) = args.get(2) {
        let postfix = postfix
            .chars()
            .nth(0)
            .unwrap();

        timer *= postfix_to_factor(postfix)
            .expect("Argument 2 should be a valid postfix");
    }

    count_down(timer);
}

