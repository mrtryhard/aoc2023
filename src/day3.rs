use grid::Grid;
use regex::Regex;

type EngineMap = Grid<char>;

fn is_symbol(c: &char) -> bool {
    !(c.is_ascii_digit() || c == &'.')
}

fn is_slice_adjacent_to_symbol(
    engine: &EngineMap,
    start_pos: (u32, u32),
    end_pos: (u32, u32),
) -> bool {
    // Assume row is the same
    assert_eq!(start_pos.0, end_pos.0);
    let row = start_pos.0 as usize;

    // First: Right and left edge + their diagonals
    // Left edge
    if start_pos.1 > 0 {
        let col = start_pos.1 as usize - 1;
        let left = engine.get(row, col);
        let left_bottom = engine.get(row + 1, col);
        let mut left_top: Option<&char> = None;

        if row > 0 {
            left_top = engine.get(row - 1, col);
        }

        if is_symbol(left.unwrap_or(&'.'))
            || is_symbol(left_bottom.unwrap_or(&'.'))
            || is_symbol(left_top.unwrap_or(&'.'))
        {
            return true;
        }
    }

    // Right edge
    {
        let col = end_pos.1 as usize;
        let right = engine.get(row, col);
        let right_bottom = engine.get(row + 1, col);
        let mut right_top: Option<&char> = None;

        if row > 0 {
            right_top = engine.get(row - 1, col);
        }

        if is_symbol(right.unwrap_or(&'.'))
            || is_symbol(right_bottom.unwrap_or(&'.'))
            || is_symbol(right_top.unwrap_or(&'.'))
        {
            return true;
        }
    }

    // Second: above and under
    //      xxxxx
    //      ooooo
    //      xxxxx
    for col in start_pos.1 as usize..=end_pos.1 as usize {
        let below = engine.get(row + 1, col);
        let mut above: Option<&char> = None;

        if row > 0 {
            above = engine.get(row - 1, col);
        }

        // The engine will return `None` when out of bounds, so we'll just pretend it's a `.`
        if is_symbol(above.unwrap_or(&'.')) || is_symbol(below.unwrap_or(&'.')) {
            return true;
        }
    }

    false
}

fn get_engine_map(input: &str) -> EngineMap {
    let mut engine = EngineMap::new(0, 0);

    input.lines().for_each(|line| {
        engine.insert_row(engine.rows(), line.trim().chars().collect());
    });

    engine
}

fn get_numbers_in_line(line: &str) -> Vec<(usize, String)> {
    let mut result = Vec::<(usize, String)>::new();
    let pattern = r"(\d+)";
    let re = Regex::new(pattern).unwrap();
    re.captures_iter(line).for_each(|capture| {
        capture.iter().skip(1).for_each(|m| {
            let m = m.unwrap();
            result.push((m.start(), line[m.start()..m.end()].to_string()));
        })
    });

    result
}

fn get_gears_in_line(line: &str) -> Vec<(usize, String)> {
    let mut result = Vec::<(usize, String)>::new();
    let pattern = r"(\*)";
    let re = Regex::new(pattern).unwrap();
    re.captures_iter(line).for_each(|capture| {
        capture.iter().skip(1).for_each(|m| {
            let m = m.unwrap();
            result.push((m.start(), line[m.start()..m.end()].to_string()));
        })
    });

    result
}

fn get_part_numbers(engine: &EngineMap) -> Vec<((u32, u32), String)> {
    let mut result = Vec::<((u32, u32), String)>::new();

    engine.iter_rows().enumerate().for_each(|(row_idx, cols)| {
        let numbers = get_numbers_in_line(&cols.into_iter().collect::<String>());

        numbers.iter().for_each(|(col_idx, number)| {
            result.push(((row_idx as u32, col_idx.clone() as u32), number.into()));
        });
    });

    result
}

fn get_gears(engine: &EngineMap) -> Vec<(u32, u32)> {
    let mut result = Vec::<(u32, u32)>::new();

    engine.iter_rows().enumerate().for_each(|(row_idx, cols)| {
        let numbers = get_gears_in_line(&cols.into_iter().collect::<String>());

        numbers.iter().for_each(|(col_idx, _)| {
            result.push((row_idx as u32, col_idx.clone() as u32));
        });
    });

    result
}

pub fn get_gears_value(input: &str) -> u32 {
    let engine = get_engine_map(&input);
    let gears = get_gears(&engine);
    let numbers = get_part_numbers(&engine);

    gears
        .iter()
        .map(|gear| {
            let adj_numbers = numbers
                .iter()
                .filter(|(coord, number)| {
                    let (gear_x, gear_y) = (gear.0 as i32, gear.1 as i32);
                    let px = coord.0 as i32;
                    let mut i = 0 as i32;

                    if (px - gear_x).abs() <= 1 {
                        while i < number.len() as i32 {
                            let py = coord.1 as i32 + i;

                            if (py - gear_y).abs() <= 1 {
                                return true;
                            }

                            i += 1;
                        }
                    }

                    false
                })
                .map(|(_, number)| number.clone())
                .collect::<Vec<String>>();

            if adj_numbers.len() == 2 {
                return adj_numbers;
            }

            return Vec::<String>::new();
        })
        .map(|numbers| {
            // An empty vector product is 1 :shrug:
            if numbers.is_empty() {
                return 0;
            }

            numbers
                .iter()
                .map(|num| num.parse::<u32>().unwrap())
                .product::<u32>()
        })
        .sum()
}

pub fn get_final_value(input: &str) -> u32 {
    let engine = get_engine_map(&input);
    let numbers = get_part_numbers(&engine);

    numbers
        .iter()
        .filter(|((x, y), number)| {
            is_slice_adjacent_to_symbol(&engine, (*x, *y), (*x, y + number.len() as u32))
        })
        .map(|(_, number)| number.parse::<u32>().unwrap())
        .sum()
}

#[cfg(test)]
mod tests_adjacence {
    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_adjacent_bottom_right() {
        let engine = get_engine_map(INPUT);
        let (row, col) = (0, 0);
        assert!(is_slice_adjacent_to_symbol(
            &engine,
            (row, col),
            (row, col + "467".len() as u32)
        ));
    }

    #[test]
    fn test_adjacent_right() {
        let engine = get_engine_map(INPUT);
        let (row, col) = (4, 0);
        assert!(is_slice_adjacent_to_symbol(
            &engine,
            (row, col),
            (row, col + "617".len() as u32)
        ));
    }

    #[test]
    fn test_adjacent_top_right() {
        let engine = get_engine_map(INPUT);
        let (row, col) = (6, 2);
        assert!(is_slice_adjacent_to_symbol(
            &engine,
            (row, col),
            (row, col + "755".len() as u32)
        ));
    }

    #[test]
    fn test_adjacent_bottom_left() {
        let engine = get_engine_map(INPUT);
        let (row, col) = (7, 6);
        assert!(is_slice_adjacent_to_symbol(
            &engine,
            (row, col),
            (row, col + "592".len() as u32)
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    fn engine_as_string(engine: &EngineMap) -> String {
        let mut result = String::new();

        engine.iter_rows().for_each(|row| {
            row.for_each(|col| result.push(col.clone()));
            result.push('\n');
        });

        result.trim().to_string()
    }

    #[test]
    fn test_get_engine_map() {
        let engine = get_engine_map(INPUT);

        assert_eq!(engine.rows(), 10);
        assert_eq!(engine.cols(), 10);
        assert_eq!(engine.get(0, 0).unwrap(), &'4');
        assert_eq!(engine.get(9, 7).unwrap(), &'8');
        assert_eq!(engine.get(4, 3).unwrap(), &'*');

        assert_eq!(INPUT, engine_as_string(&engine));
    }

    #[test]
    fn test_is_symbol() {
        assert_eq!(is_symbol(&'.'), false);
        assert_eq!(is_symbol(&'6'), false);
        assert_eq!(is_symbol(&'0'), false);
        assert_eq!(is_symbol(&'*'), true);
        assert_eq!(is_symbol(&'&'), true);
        assert_eq!(is_symbol(&'%'), true);
        assert_eq!(is_symbol(&'$'), true);
        assert_eq!(is_symbol(&'/'), true);
        assert_eq!(is_symbol(&'@'), true);
        assert_eq!(is_symbol(&' '), true);
    }

    #[test]
    fn test_get_game_set() {
        let value = get_final_value(INPUT);
        assert_eq!(value, 4361);
    }

    #[test]
    fn test_get_part_numbers() {
        let engine = get_engine_map(INPUT);
        let numbers = get_part_numbers(&engine);

        assert_eq!(numbers.len(), 10);
        assert_eq!(numbers[0], ((0, 0), "467".to_owned()));
        assert_eq!(numbers[9], ((9, 5), "598".to_owned()));
    }

    #[test]
    fn test_get_line_numbers() {
        let result = get_numbers_in_line("..467..488..");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], (2, "467".to_owned()));
        assert_eq!(result[1], (7, "488".to_owned()));
    }

    #[test]
    fn test_gears_output() {
        let result = get_gears_value(INPUT);
        assert_eq!(result, 467835);
    }
}
