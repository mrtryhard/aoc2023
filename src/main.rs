use std::fs::File;
use std::io::Read;

mod day1;
mod day2;
mod day3;
mod day4;

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

fn solve_day3() {
    let mut file = File::open("day3.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let answer = day3::get_final_value(contents.as_str());
    let answer2 = day3::get_gears_value(contents.as_str());
    println!("Day3: {answer}");
    println!("Day3 Gears: {answer2}");
}

fn solve_day4() {
    let mut file = File::open("day4.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let answer = day4::solve_scratchcards(contents.as_str());
    let answer2 = day4::solve_bonus_total_scratchcards(contents.as_str());
    println!("Day4: {answer}");
    println!("Day4 cards: {answer2}");
}

fn main() {
    //solve_day1();
    //solve_day2();
    //solve_day3();
    solve_day4();
}
