
use std::io::Write;
use std::time::{Instant, Duration};
use std::thread::sleep;


fn get_hours_passed(ms: u128) -> u128 {ms / 360000}
fn get_minutes_passed(ms: u128) -> u128 {(ms % 360000) / 6000}
fn get_seconds_passed(ms: u128) -> u128 {(ms % 6000) / 100}
fn get_millis_passed(ms: u128) -> u128  {ms % 100}


fn main() {
    let start_time = Instant::now();

    loop {
        let time_passed = start_time.elapsed().as_millis() / 10;

        print!("\r{:02}:{:02}:{:02}:{:02}",
               get_hours_passed(time_passed),
               get_minutes_passed(time_passed),
               get_seconds_passed(time_passed),
               get_millis_passed(time_passed),
            );
        std::io::stdout()
            .flush()
            .expect("Stdout should be flush-able");

        sleep(Duration::from_millis(10));
    }
}

