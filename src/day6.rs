const MY_BOAT_ACCELERATION: u64 = 1;

fn parse_record_entry(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (times_str, distances_str) = input.trim().split_once('\n').unwrap();

    println!("Times={times_str}");
    let times = times_str
        .split(' ')
        .skip(1)
        .filter(|c| !c.trim().is_empty())
        .map(|num| num.trim().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let distances = distances_str
        .split(' ')
        .skip(1)
        .filter(|c| !c.trim().is_empty())
        .map(|num| num.trim().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    (times, distances)
}

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

fn parse_record_entry_single(input: &str) -> (u64, u64) {
    let (times_str, distances_str) = input.trim().split_once('\n').unwrap();

    let (_, times) = times_str.split_once(' ').unwrap();
    let mut time = times.to_owned();
    remove_whitespace(&mut time);

    let (_, distances) = distances_str.split_once(' ').unwrap();
    let mut distance = distances.to_owned();
    remove_whitespace(&mut distance);

    (time.parse::<u64>().unwrap(), distance.parse::<u64>().unwrap())
}

fn possible_wins_for_record(distance: u64, time_max: u64) -> Vec<u64> {
    let mut wins = Vec::<u64>::new();

    for time_hold in 1..time_max - 1 {
        let distance_done = time_hold * MY_BOAT_ACCELERATION * (time_max - time_hold);

        if distance_done > distance {
            wins.push(time_hold);
        }
    }

    wins
}

pub fn get_wins_product(input: &str) -> u32 {
    let (times, distances) = parse_record_entry(input);
    assert_eq!(times.len(), distances.len());

    let mut product = 1;
    for idx in 0..times.len() {
        product *= possible_wins_for_record(distances[idx] as u64, times[idx] as u64).len() as u32;
    }

    product
}

pub fn get_wins_product_2(input: &str) -> u64 {
    let (time, distance) = parse_record_entry_single(input);

    possible_wins_for_record(distance, time).len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    // Line 1: Seeds needed to be planted
    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_load() {
        let product = get_wins_product(INPUT);

        assert_eq!(product, 288);
    }

    #[test]
    fn test_possible_wins_for_record() {
        let possibilities = possible_wins_for_record(9, 7);
        assert_eq!(possibilities.len(), 4);

        let possibilities = possible_wins_for_record(40, 15);
        assert_eq!(possibilities.len(), 8);

        let possibilities = possible_wins_for_record(200, 30);
        assert_eq!(possibilities.len(), 9);
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
        let entry = parse_record_entry_single("Time:      7  15   30
Distance:  9  40  200");
        let product = possible_wins_for_record(entry.1, entry.0);

        assert_eq!(product.len(), 71503);
    }
}
