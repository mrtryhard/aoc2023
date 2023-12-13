pub fn solve_scratchcards(input: &str) -> u32 {
    get_cards(input).iter().map(calculate_points_card).sum()
}

// Returns: number of total scratch cards after bonuses
pub fn solve_bonus_total_scratchcards(input: &str) -> u32 {
    let cards = get_cards(input);

    // Vector of winnings
    //      [card 1, card 2, ...]
    // e.g. [     4,      2, ...]
    let wins_per_card = cards
        .iter()
        .map(calculate_times_won_card)
        .collect::<Vec<u32>>();
    let mut cards_count = Vec::<u32>::new();
    cards_count.resize(cards.len(), 1); // 1 card each

    // Process per card and changes the amount per card.
    wins_per_card
        .iter()
        .enumerate()
        .for_each(|(idx, numbers_won)| {
            // Update the next cards count
            let bound = *numbers_won as usize;
            for update_idx in 0..bound {
                cards_count[idx + 1 + update_idx] += cards_count[idx];
            }
        });

    cards_count.iter().sum()
}

struct Card {
    id: u32,
    numbers: Vec<u32>,
    wins: Vec<u32>,
}

fn calculate_times_won_card(card: &Card) -> u32 {
    card.numbers
        .iter()
        .filter(|num| card.wins.contains(num))
        .count() as u32
}

fn calculate_points_card(card: &Card) -> u32 {
    let won_count = card
        .numbers
        .iter()
        .filter(|num| card.wins.contains(num))
        .count() as u32;

    match won_count {
        0 => 0,
        _ => 2u32.pow(won_count - 1),
    }
}

fn get_cards(input: &str) -> Vec<Card> {
    input.lines().map(get_card_info).collect()
}

fn get_card_info(input: &str) -> Card {
    let mut id_and_numbers = input.split(':');
    let id = id_and_numbers
        .next()
        .unwrap()
        .split(' ')
        .filter(|c| !c.trim().is_empty())
        .nth(1)
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let num_and_wins = id_and_numbers
        .next()
        .unwrap()
        .split('|')
        .collect::<Vec<&str>>();

    let numbers = num_and_wins[0]
        .split(' ')
        .filter(|c| !c.is_empty())
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let wins = num_and_wins[1]
        .split(' ')
        .filter(|c| !c.is_empty())
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    Card { id, numbers, wins }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_get_card_info() {
        let expected_nums: Vec<u32> = vec![13, 32, 20, 16, 61];
        let expected_wins: Vec<u32> = vec![61, 30, 68, 82, 17, 32, 24, 19];

        let card = get_card_info("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19");

        assert_eq!(card.id, 2);
        assert_eq!(card.numbers, expected_nums);
        assert_eq!(card.wins, expected_wins);
    }

    #[test]
    fn test_get_cards() {
        let cards = get_cards(INPUT);
        let expected_nums: Vec<u32> = vec![13, 32, 20, 16, 61];
        let expected_wins: Vec<u32> = vec![61, 30, 68, 82, 17, 32, 24, 19];

        assert_eq!(cards.len(), 6);
        assert_eq!(cards[1].numbers, expected_nums);
        assert_eq!(cards[1].wins, expected_wins);
    }

    #[test]
    fn test_get_card_points_full() {
        let card = Card {
            id: 1,
            numbers: vec![41, 48, 83, 86, 17],
            wins: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };

        let points = calculate_points_card(&card);
        assert_eq!(points, 8);
    }

    #[test]
    fn test_get_card_points_min() {
        let card = Card {
            id: 4,
            numbers: vec![41, 92, 73, 84, 69],
            wins: vec![59, 84, 76, 51, 58, 5, 54, 83],
        };

        let points = calculate_points_card(&card);
        assert_eq!(points, 1);
    }

    #[test]
    fn test_get_card_points_zero_match() {
        let card = Card {
            id: 5,
            numbers: vec![41, 92, 73, 84, 69],
            wins: vec![59, 3, 76, 51, 58, 5, 54, 83],
        };

        let points = calculate_points_card(&card);
        assert_eq!(points, 0);
    }

    #[test]
    fn test_get_scratchcards_total() {
        let total = solve_scratchcards(INPUT);
        assert_eq!(total, 13);
    }

    #[test]
    fn test_get_total_scratchcards_won() {
        let total = solve_bonus_total_scratchcards(INPUT);
        assert_eq!(total, 30);
    }
}
