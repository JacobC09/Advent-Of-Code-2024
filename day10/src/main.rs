use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

#[allow(dead_code)]
const TEST_INPUT: &str = r#"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"#;

fn main() {
    println!("Day 10");
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

struct Map {
    grid: Vec<Vec<u8>>,
    heads: Vec<(i32, i32)>,
    width: i32,
    height: i32,
}

impl Map {
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<u8>> = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.chars()
                    .map(|ch| {
                        if ch == '.' {
                            10
                        } else {
                            ch.to_digit(10).unwrap() as u8
                        }
                    })
                    .collect()
            })
            .collect();

        
        let mut heads = Vec::new();
        for (y, row) in grid.iter().enumerate() {
            for (x, level) in row.iter().enumerate() {
                if *level == 0 {
                    heads.push((x as i32, y as i32));
                }
            }
        }

        let width = if let Some(first) = grid.first() {
            first.len()
        } else {
            0
        } as i32;

        let height = grid.len() as i32;

        Self {
            grid,
            heads,
            width,
            height,
        }
    }

    fn in_bounds(&self, pos: (i32, i32)) -> bool {
        pos.0 >= 0 && pos.0 < self.width && pos.1 >= 0 && pos.1 < self.height
    }

    fn at(&self, pos: (i32, i32)) -> u8 {
        self.grid[pos.1 as usize][pos.0 as usize]
    }
}

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn part_one(input: &str) -> i32 {
    let map = Map::new(input);
    let mut total_score = 0;

    for head in &map.heads {
        let mut ends = HashSet::new();
        let mut queue = vec![*head];

        while let Some(pos) = queue.pop() {
            let level = map.at(pos);

            for dir in DIRECTIONS.iter() {
                let next = (pos.0 + dir.0, pos.1 + dir.1);

                if map.in_bounds(next) && map.at(next) == level + 1 {
                    if level == 8 {
                        ends.insert(next);
                    } else {
                        queue.push(next);
                    }
                }
            }
        }

        total_score += ends.len() as i32;
    }

    total_score
}

fn part_two(input: &str) -> i32 {
    let map = Map::new(input);
    let mut total_score = 0;

    for head in &map.heads {
        let mut queue = vec![*head];

        while let Some(pos) = queue.pop() {
            let level = map.at(pos);

            for dir in DIRECTIONS.iter() {
                let next = (pos.0 + dir.0, pos.1 + dir.1);

                if map.in_bounds(next) && map.at(next) == level + 1 {
                    if level == 8 {
                        total_score += 1;
                    } else {
                        queue.push(next);
                    }
                }
            }
        }
    }

    total_score
}

#[cfg(test)]
mod test {
    use super::*;
    use utils::test_part;

    test_part!(part_one, TEST_INPUT, 36);
    test_part!(part_two, TEST_INPUT, 81);
}
