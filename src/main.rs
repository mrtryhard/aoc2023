use std::fs::File;
use std::io::Read;

mod day1;

fn solve_day1() {
    let mut file = File::open("day1.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let value = day1::get_calibration_value(contents.to_ascii_lowercase().as_str());

    println!("Day1: {value}");
}

fn main() {
    solve_day1();
}
