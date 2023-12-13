use num::Integer;
use std::io::Write;
use std::ops::{Deref, DerefMut, Index};
use std::str::FromStr;

type NodePosition = Option<usize>;

#[derive(PartialEq, Debug)]
struct Node {
    id: String,
    left: NodePosition,
    right: NodePosition,
}

struct Map {
    moves: String,
    nodes: Vec<Node>,
    start: String,
}

struct MapIterator<'a> {
    pattern: &'a str,
    nodes: &'a Vec<Node>,
    current_pattern_index: usize,
    current_node: NodePosition,
}

impl Map {
    pub fn iter(&self) -> MapIterator {
        MapIterator {
            pattern: &self.moves,
            nodes: &self.nodes,
            current_pattern_index: 0,
            current_node: self.nodes.iter().position(|node| node.id == self.start),
        }
    }

    // Simply push a new node.
    pub fn push(&mut self, id: &str) {
        self.nodes.push(Node {
            id: id.to_owned(),
            left: None,
            right: None,
        });
    }

    // Links a node with present nodes in the map.
    // A node may link to itself.
    pub fn link(&mut self, id: &str, left: &str, right: &str) {
        let node2update = self.nodes.iter().position(|node| node.id == id).unwrap();

        self.nodes[node2update].left = self.nodes.iter().position(|node| node.id == left);
        self.nodes[node2update].right = self.nodes.iter().position(|node| node.id == right);
    }

    fn new(input: &str, start: &str) -> Map {
        let (moves, move_map) = input.trim().split_once('\n').unwrap();

        let mut map = Map {
            moves: moves.trim().to_owned(),
            nodes: vec![],
            start: start.to_owned(),
        };

        // Push existing nodes
        move_map
            .lines()
            .filter(|line| !line.trim().is_empty())
            .for_each(|line| {
                let (id, _) = line.trim().split_once(' ').unwrap();
                map.push(id);
            });

        // Link nodes
        move_map
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(parse_map_entry)
            .for_each(|(id, left, right)| {
                map.link(id.as_ref(), left.as_ref(), right.as_ref());
            });

        map
    }
}

impl<'a> Iterator for MapIterator<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        let result_node = self.nodes.get(self.current_node.unwrap_or(0));

        match self
            .pattern
            .chars()
            .nth(self.current_pattern_index)
            .unwrap()
        {
            'R' => self.current_node = result_node.unwrap().right,
            _ => self.current_node = result_node.unwrap().left,
        }

        if self.current_pattern_index == self.pattern.len() - 1 {
            self.current_pattern_index = 0;
        } else {
            self.current_pattern_index += 1;
        }

        if result_node.unwrap().id.as_str().ends_with("Z") {
            None
        } else {
            result_node
        }
    }
}

// Returns (id, left, right)
fn parse_map_entry(input: &str) -> (String, String, String) {
    let pattern = r"\s*(?P<idGroup>\w+)\s*=\s*\((?P<leftGroup>\w+)\s*,\s*(?P<rightGroup>\w+)\)";
    let reg = regex::Regex::new(pattern).unwrap();
    let captures = reg.captures(input).unwrap();

    assert_eq!(captures.len(), 4); // 3 matches + first which is the full input

    (
        captures.index(1).to_owned(),
        captures.index(2).to_owned(),
        captures.index(3).to_owned(),
    )
}

fn parse_map(input: &str) -> Map {
    Map::new(input, "AAA")
}

fn parse_maps(input: &str) -> Vec<Map> {
    let mut maps: Vec<Map> = Vec::new();
    let (_, move_map) = input.trim().split_once('\n').unwrap();

    move_map
        .lines()
        .filter(|line| !line.trim().is_empty())
        .enumerate()
        .for_each(|(id, line)| {
            let m = &line.trim()[0..3];

            if m.ends_with("A") {
                maps.push(Map::new(&input, m));
            }
        });

    maps
}

pub fn solve1(input: &str) -> usize {
    parse_map(input).iter().count()
}

pub fn solve2(input: &str) -> u64 {
    let maps = parse_maps(input);

    maps.iter()
        .map(|p| p.iter().count())
        .reduce(|a, b| a.lcm(&b))
        .unwrap() as u64
}

#[cfg(test)]
mod tests_iterator {
    use super::*;

    const INPUT_6_STEPS: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const INPUT_2_STEPS_NOT_START: &str = "RL

BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
AAA = (BBB, CCC)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_basic_iterator() {
        let map = parse_map(&INPUT_6_STEPS);
        let steps = map.iter().count();

        assert_eq!(steps, 6);
    }

    #[test]
    fn test_iterator_when_aaa_is_not_first_line() {
        let map = parse_map(&INPUT_2_STEPS_NOT_START);

        assert_eq!(map.nodes[3].id, "AAA");
        assert_eq!(map.nodes[map.nodes[3].left.unwrap()].id, "BBB");
        assert_eq!(map.nodes[map.nodes[3].right.unwrap()].id, "CCC");

        let steps = map.iter().count();

        assert_eq!(steps, 2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 2 steps
    const INPUT_2_STEPS: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const INPUT_6_STEPS: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const BONUS: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_line_parse_happy_path() {
        let input = "AAA = (BBB, CCC)";
        let (id, left, right) = parse_map_entry(input);

        assert_eq!(id, "AAA");
        assert_eq!(left, "BBB");
        assert_eq!(right, "CCC");
    }

    #[test]
    fn test_line_parse_lot_of_spaces_path() {
        let input = " AAA   =  (BBB   ,       CCC) ";
        let (id, left, right) = parse_map_entry(input);

        assert_eq!(id, "AAA");
        assert_eq!(left, "BBB");
        assert_eq!(right, "CCC");
    }

    #[test]
    fn test_parse_map_happy_path() {
        let map = parse_map(INPUT_2_STEPS);

        assert_eq!(map.moves, "RL");
        assert_eq!(map.nodes.len(), 7);
        assert_eq!(map.nodes[0].id, "AAA");
        assert_eq!(map.nodes[map.nodes[0].left.unwrap()].id, "BBB");
        assert_eq!(map.nodes[map.nodes[0].right.unwrap()].id, "CCC");
        assert_eq!(map.nodes[6].id, "ZZZ");
        assert_eq!(map.nodes[map.nodes[6].left.unwrap()].id, "ZZZ");
        assert_eq!(map.nodes[map.nodes[6].right.unwrap()].id, "ZZZ");
    }

    #[test]
    fn test_parse_maps() {
        let maps = parse_maps(BONUS);

        assert_eq!(maps.len(), 2);
        assert_eq!(maps[0].start, "11A");
        assert_eq!(maps[1].start, "22A");
    }

    #[test]
    fn test_solve2() {
        let maps = solve2(BONUS);

        assert_eq!(maps, 6);
    }
}
