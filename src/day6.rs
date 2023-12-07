fn parse_record_entry(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (times_str, distances_str) = input.trim().split_once('\n').unwrap();

    let times = times_str
        .split(' ')
        .filter_map(|c| c.trim().parse::<u32>().ok())
        .collect::<Vec<u32>>();

    let distances = distances_str
        .split(' ')
        .filter_map(|c| c.trim().parse::<u32>().ok())
        .collect::<Vec<u32>>();

    (times, distances)
}

fn parse_record_entry_single(input: &str) -> (u64, u64) {
    let (times_str, distances_str) = input.trim().split_once('\n').unwrap();
    let (_, times) = times_str.split_once(' ').unwrap();
    let (_, distances) = distances_str.split_once(' ').unwrap();

    (
        times.replace(" ", "").trim().parse::<u64>().unwrap(),
        distances.replace(" ", "").trim().parse::<u64>().unwrap(),
    )
}

fn possible_wins_for_record(distance_to_beat: u64, time_max: u64) -> u64 {
    (1..time_max - 1)
        .map(|time_hold| time_hold * (time_max - time_hold))
        .filter(|distance| distance_to_beat < *distance)
        .count() as u64
}

pub fn get_wins_product(input: &str) -> u64 {
    let (times, distances) = parse_record_entry(input);
    assert_eq!(times.len(), distances.len());

    let mut product = 1;
    for idx in 0..times.len() {
        product *= possible_wins_for_record(distances[idx] as u64, times[idx] as u64);
    }

    product
}

pub fn get_possible_wins_single(input: &str) -> u64 {
    let (time, distance) = parse_record_entry_single(input);

    possible_wins_for_record(distance, time)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Line 1: Seeds needed to be planted
    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";
    const INPUT_BONUS: &str = "Time:        40     92     97     90
Distance:   215   1064   1505   1100";

    #[test]
    fn test_load() {
        let product = get_wins_product(INPUT);

        assert_eq!(product, 288);
    }

    #[test]
    fn test_possible_wins_for_record() {
        let possibilities = possible_wins_for_record(9, 7);
        assert_eq!(possibilities, 4);

        let possibilities = possible_wins_for_record(40, 15);
        assert_eq!(possibilities, 8);

        let possibilities = possible_wins_for_record(200, 30);
        assert_eq!(possibilities, 9);
    }

    #[test]
    fn test_parse_record_entry() {
        let (times, distances) = parse_record_entry(INPUT);

        let expected_times = vec![7, 15, 30];
        let expected_distances = vec![9, 40, 200];

        assert_eq!(times, expected_times);
        assert_eq!(distances, expected_distances);
    }

    #[test]
    fn test_parse_record_entry_single() {
        let (time, distance) = parse_record_entry_single(INPUT);

        assert_eq!(time, 71530);
        assert_eq!(distance, 940200);
    }

    #[test]
    fn test_get_wins_product_2() {
        let (time, distance) = parse_record_entry_single(INPUT_BONUS);
        let product = possible_wins_for_record(distance, time);

        assert_eq!(product, 28545089);
    }
}
