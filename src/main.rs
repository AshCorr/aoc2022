use std::env;
use std::fs::File;
use std::io::Read;

mod day_1;

type DayFunction = fn(String) -> String;

fn main() {
    let days: [DayFunction; 1] = [day_1::day];

    let args: Vec<String> = env::args().collect();
    let day: usize = args[1].parse().expect("Day should be a numerical value");

    let mut input = String::new();
    File::open(format!("data/{}", day))
        .expect("Failed to read input file, right day entered?")
        .read_to_string(&mut input)
        .expect("Failed to read input file, right day entered?");

    println!("Result: {}", days[day - 1](input));
}
