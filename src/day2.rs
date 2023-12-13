use regex::Regex;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;

#[derive(Debug, PartialEq, Eq)]
struct ParseGameSetError;

struct Game {
    pub sets: Vec<GameSet>,
    pub id: u32,
}

struct GameSet {
    pub red: u32,
    pub blue: u32,
    pub green: u32,
}

impl FromStr for GameSet {
    type Err = ParseGameSetError;

    fn from_str(set: &str) -> Result<Self, Self::Err> {
        let pattern = r"(?P<redGroup>\d+ red)|(?P<greenGroup>\d+ green)|(?P<blueGroup>\d+ blue)";
        let re = Regex::new(pattern).unwrap();
        let mut result = GameSet {
            red: 0,
            blue: 0,
            green: 0,
        };

        let group_names: Vec<&str> = re.capture_names().skip(1).map(|x| x.unwrap()).collect();

        for caps in re.captures_iter(set) {
            for name in &group_names {
                if let Some(m) = caps.name(name) {
                    let digit_ends = m.as_str().find(' ').unwrap();
                    let value = m.as_str()[..digit_ends].parse::<u32>().unwrap();

                    match name {
                        &"greenGroup" => result.green = value,
                        &"blueGroup" => result.blue = value,
                        &"redGroup" => result.red = value,
                        _ => panic!("Unknown color group {name}"),
                    }
                }
            }
        }

        Ok(result)
    }
}

impl Game {
    pub fn satisfies_constraints(&self, blue: u32, red: u32, green: u32) -> bool {
        !self
            .sets
            .iter()
            .any(|set| set.blue > blue || set.red > red || set.green > green)
    }
}

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let bound = input.find(':').unwrap();

        Ok(Game {
            id: input[5..bound].parse::<u32>().unwrap(),
            sets: get_game_sets_str(input)
                .iter()
                .map(|set| GameSet::from_str(set).unwrap())
                .collect::<Vec<GameSet>>(),
        })
    }
}

pub fn get_valid_games_id_sums(input: &str, blue: u32, red: u32, green: u32) -> u32 {
    get_games(input)
        .iter()
        .filter(|game| game.satisfies_constraints(blue, red, green))
        .map(|game| game.id)
        .sum()
}

pub fn get_games_power(input: &str) -> u32 {
    get_games(input)
        .iter()
        .map(|game| {
            let blue = game.sets.iter().max_by_key(|set| set.blue).unwrap().blue;
            let green = game.sets.iter().max_by_key(|set| set.green).unwrap().green;
            let red = game.sets.iter().max_by_key(|set| set.red).unwrap().red;

            blue * green * red
        })
        .sum()
}

fn get_games(input: &str) -> Vec<Game> {
    let games_str: Vec<&str> = input.lines().collect();

    games_str
        .iter()
        .map(|game| Game::from_str(game).unwrap())
        .collect::<Vec<Game>>()
}

fn get_game_sets_str(game: &str) -> Vec<&str> {
    let sets_start = game.find(':').unwrap() + 1; // Exclude the char itself.
    let sets = game[sets_start..].split(';').collect();

    sets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_game_set() {
        let line = "3 blue, 4 red";
        let game_set = GameSet::from_str(line).unwrap();
        assert_eq!(game_set.blue, 3);
        assert_eq!(game_set.red, 4);
        assert_eq!(game_set.green, 0);
    }

    #[test]
    fn test_get_game_sets() {
        let sets = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let values = get_game_sets_str(sets);

        assert_eq!(values[0], " 3 blue, 4 red");
        assert_eq!(values[1], " 1 red, 2 green, 6 blue");
        assert_eq!(values[2], " 2 green");
    }

    #[test]
    fn test_parse_games_log() {
        const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let values = get_games(&INPUT);

        assert_eq!(values.len(), 5);
        assert_eq!(values[0].id, 1);
        assert_eq!(values[4].id, 5);
        assert_eq!(values[0].sets[0].blue, 3);
        assert_eq!(values[0].sets[0].red, 4);
        assert_eq!(values[0].sets[1].red, 1);
        assert_eq!(values[0].sets[1].green, 2);
        assert_eq!(values[0].sets[1].blue, 6);
        assert_eq!(values[4].sets[0].red, 6);
        assert_eq!(values[4].sets[0].blue, 1);
        assert_eq!(values[4].sets[0].green, 3);
    }

    #[test]
    fn test_valid_games_id_sum() {
        const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let sum = get_valid_games_id_sums(&INPUT, 14, 12, 13);

        assert_eq!(sum, 8);
    }

    #[test]
    fn test_games_power_sum() {
        const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let sum = get_games_power(&INPUT);

        assert_eq!(sum, 2286);
    }

    #[test]
    fn test_game_satisfies_constraints() {
        const INPUT: &str = "Game 1: 3 blue, 4 red";
        let values = get_games(&INPUT);

        assert!(!values[0].satisfies_constraints(4, 1, 0)); // Fails because red.
        assert!(values[0].satisfies_constraints(4, 5, 1)); // Passes.
        assert!(!values[0].satisfies_constraints(2, 5, 0)); // Fails because blue.
        assert!(values[0].satisfies_constraints(3, 4, 0)); // Pass.
    }
}
