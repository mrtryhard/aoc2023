const DIGITS: [&str; 20] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2",
    "3", "4", "5", "6", "7", "8", "9",
];

fn convert_digit(digit: &str) -> u32 {
    match digit {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => digit.parse::<u32>().unwrap_or(0),
    }
}

fn get_line_value(line: &str) -> u32 {
    let mut first_digit = 0;
    let mut first_pos = line.len();
    let mut last_digit = 0;
    let mut last_pos: Option<usize> = None;

    for digit in DIGITS {
        if let Some(pos) = line.find(digit) {
            if pos < first_pos {
                first_digit = convert_digit(digit);
                first_pos = pos;
            }
        }

        if let Some(pos) = line.rfind(digit) {
            if last_pos.is_none() || pos > last_pos.unwrap() {
                last_digit = convert_digit(digit);
                last_pos = Some(pos);
            }
        }
    }

    format!("{first_digit}{last_digit}").parse::<u32>().unwrap()
}

pub fn get_calibration_value(block: &str) -> u32 {
    block.lines().map(|line| get_line_value(line)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_calibration_value_simple() {
        let line = "1abc2";
        let value = get_line_value(line);
        assert_eq!(value, 12);
    }

    #[test]
    fn test_line_calibration_value_simple_digit_letters() {
        let line = "1abc2zero";
        let value = get_line_value(line);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_line_calibration_value_simple_digit_letters_digit_between() {
        let line = "one2one";
        let value = get_line_value(line);
        assert_eq!(value, 11);
    }

    #[test]
    fn test_line_calibration_value_digit_wrapped_by_letters() {
        let line = "pqr3stu8vwx";
        let value = get_line_value(line);
        assert_eq!(value, 38);
    }

    #[test]
    fn test_line_calibration_value_only_one_digit_wrapped_by_letters() {
        let line = "treb7uchet";
        let value = get_line_value(line);
        assert_eq!(value, 77);
    }

    #[test]
    fn test_line_calibration_value_with_letters_digit() {
        let line = "fourtreb7uchet";
        let value = get_line_value(line);
        assert_eq!(value, 47);
    }

    #[test]
    fn test_input_calibration_value_happy() {
        let input: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let value = get_calibration_value(input);
        assert_eq!(value, 142);
    }

    #[test]
    fn test_input_calibration_value_happy_with_letters_digit() {
        let input: &str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        let value = get_calibration_value(input);
        assert_eq!(value, 281);
    }

    #[test]
    fn test_input_calibration_value_empty() {
        // Assumption: Empty input gives sum of 0.
        let input: &str = "";
        let value = get_calibration_value(input);
        assert_eq!(value, 0);
    }

    #[test]
    fn test_input_calibration_value_only_newlines() {
        // Assumption: Empty but newlines only input gives sum of 0.
        let input: &str = "\n\n\n";
        let value = get_calibration_value(input);
        assert_eq!(value, 0);
    }
}
