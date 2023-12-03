use std::fs::File;
use std::io::Read;

mod day1;
mod day2;

fn solve_day1() {
    let mut file = File::open("day1.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let value = day1::get_calibration_value(contents.to_ascii_lowercase().as_str());

    println!("Day1: {value}");
}

fn solve_day2() {
    let mut file = File::open("day2.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let value_valid_sum =
        day2::get_valid_games_id_sums(contents.to_ascii_lowercase().as_str(), 14, 12, 13);
    let value_power = day2::get_games_power(contents.to_ascii_lowercase().as_str());

    println!("Day2 valid sum: {value_valid_sum}");
    println!("Day2 power: {value_power}");
}

fn main() {
    solve_day1();
    solve_day2();
}
