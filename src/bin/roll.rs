
use rand::Rng;


fn main() {

    let args: Vec<String> = std::env::args()
        .skip(1)
        .collect();

    let error_msg = "the arguments should be numbers";
    let min;
    let max;

    match args.len() {
        0 => {
            min = 1;
            max = 100;
        },
        1 => {
            min = 1;
            max = args[0].parse().expect(error_msg);
        },
        2 => {
            min = args[0].parse().expect(error_msg);
            max = args[1].parse().expect(error_msg);
        },
        _ => {
            panic!("number of arguments should not exceed 2");
        }
    }

    if max < min {
        panic!("max should not be less than min");
    }

    let random_number = rand::thread_rng().gen_range(min..=max);
    println!("{random_number}");
}

