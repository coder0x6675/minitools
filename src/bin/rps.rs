
use std::str::FromStr;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

// ---

#[derive(Debug, PartialEq)]
enum Choice {
	Rock,
	Paper,
	Scissor,
}

impl std::fmt::Display for Choice {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self {
			Self::Rock    => "rock",
			Self::Paper   => "paper",
			Self::Scissor => "scissor",
		})
	}
}

impl FromStr for Choice {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"r" | "rock"    => Ok(Self::Rock),
			"p" | "paper"   => Ok(Self::Paper),
			"s" | "scissor" => Ok(Self::Scissor),
			_  => Err(()),
		}
	}
}

impl Distribution<Choice> for Standard {
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Choice {
		match rng.gen_range(0..=2) {
			0 => Choice::Rock,
			1 => Choice::Paper,
			2 => Choice::Scissor,
			_ => panic!("invalid random value generated"),
		}
	}
}

impl Choice {

	fn beats(&self, other: &Self) -> bool {
		if *self == Self::Rock    && *other == Self::Scissor { return true; }
		if *self == Self::Paper   && *other == Self::Rock    { return true; }
		if *self == Self::Scissor && *other == Self::Paper   { return true; }
		return false;
	}

	fn evens(&self, other: &Self) -> bool {
		self == other
	}

	fn drops(&self, other: &Self) -> bool {
		!self.evens(&other) && !self.beats(&other)
	}
}

// ---

fn main() {
	
	let user: Choice = std::env::args()
		.nth(1)
		.expect("Specify your choice as the first argument!")
		.parse()
		.expect("Valid choices: rock, paper or scissor!");
	println!("You chose: {user}");

	let computer: Choice = rand::random();
	println!("Computer chose: {computer}");

	if user.evens(&computer) { println!("Its a tie!") ; std::process::exit(2) }
	if user.drops(&computer) { println!("You lose!")   ; std::process::exit(1) }
	if user.beats(&computer) { println!("You win!")    ; std::process::exit(0) }
}

