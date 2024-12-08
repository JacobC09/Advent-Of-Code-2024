use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");

#[allow(dead_code)]
const TEST_INPUT: &str = r#"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#;

fn main() {
    println!("Day 8:");
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

struct Map {
    width: i32,
    height: i32,
    antennas: HashMap<char, Vec<(i32, i32)>>
}

impl Map {
    fn new(input: &str) -> Self {
        let mut antennas = HashMap::new();
        let map: Vec<&str> = input
            .lines()
            .filter(|line| !line.is_empty())
            .collect();
        let height = map.len() as i32;
        let width = if let Some(first) = map.first() { first.len() } else { 0 } as i32;

        for (y, line) in map.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != '.' {
                    antennas
                        .entry(ch)
                        .or_insert_with(Vec::new)
                        .push((x as i32, y as i32));
                }
            }
        }

        Self {
            width,
            height,
            antennas
        }
    }

    fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }
}

fn part_one(input: &str) -> i32 {
    let map = Map::new(input);
    let mut antinodes = HashSet::new();

    for locations in map.antennas.values() {
        for loc in locations {
            for other in locations {
                if other == loc {
                    continue;
                }
                
                let x = loc.0 * 2 - other.0;
                let y =  loc.1 * 2 - other.1;

                if map.in_bounds(x, y) {
                    antinodes.insert((x, y));
                }
            }
        }
    }

    antinodes.len() as i32
}

fn part_two(input: &str) -> i32 {
    let map = Map::new(input);
    let mut antinodes = HashSet::new();

    for locations in map.antennas.values() {
        for loc in locations {
            for other in locations {
                if other == loc {
                    continue;
                }

                let mx = loc.0 - other.0;
                let my = loc.1 - other.1;
                let mut x = loc.0 - mx;
                let mut y =  loc.1  - my;

                while map.in_bounds(x, y) {
                    antinodes.insert((x, y));

                    x -= mx;
                    y -= my;
                }
            }
        }
    }

    antinodes.len() as i32
}

#[cfg(test)]
mod test {
    use super::*;
    use utils::test_part;

    test_part!(part_one, TEST_INPUT, 14);
    test_part!(part_two, TEST_INPUT, 34);
}
