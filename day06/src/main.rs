use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

#[allow(dead_code)]
const TEST_INPUT: &str = r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;

fn main() {
    println!("Day 6:");
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

// fn print_map(map: &Vec<Vec<bool>>, pos: (i32, i32)) {
//     println!("\nMap with pos: {:?}", pos);
//     for (y, row) in map.iter().enumerate() {
//         for (x, tile) in row.iter().enumerate() {
//             if pos == (x as i32, y as i32) {
//                 print!("O");
//             } else if *tile {
//                 print!("#")
//             } else {
//                 print!(".");
//             }
//         }
//         println!();
//     }
// }

fn parse_input(input: &str) -> ((i32, i32), Vec<Vec<bool>>) {
    let mut start = (0, 0);

    let map = input
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, ch)| {
                    if ch == '^' {
                        start = (x as i32, y as i32);
                    }

                    ch == '#'
                })
                .collect()
        })
        .collect();

    (start, map)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vector {
    pos: (i32, i32),
    dir: (i32, i32),
}

fn rotate(vector: (i32, i32)) -> (i32, i32) {
    (-vector.1, vector.0)
}

fn in_bounds(pos: (i32, i32), width: i32, height: i32) -> bool {
    pos.0 >= 0 && pos.0 < width && pos.1 >= 0 && pos.1 < height
}

fn get_path(start: (i32, i32), map: &Vec<Vec<bool>>) -> HashSet<(i32, i32)> {
    let mut path = HashSet::new();
    let mut v = Vector {
        pos: start,
        dir: (0, -1),
    };

    let height = map.len() as i32;
    let width = if map.len() > 0 { map[0].len() } else { 0 } as i32;

    loop {
        path.insert(v.pos);

        v.pos.0 += v.dir.0;
        v.pos.1 += v.dir.1;

        if !in_bounds(v.pos, width, height) {
            break;
        }

        if map[v.pos.1 as usize][v.pos.0 as usize] {
            v.pos.0 -= v.dir.0;
            v.pos.1 -= v.dir.1;
            v.dir = rotate(v.dir);
            continue;
        }
    }

    path
}

fn part_one(input: &str) -> i32 {
    let (start, map) = parse_input(input);

    get_path(start, &map).len() as i32
}

fn part_two(input: &str) -> i32 {
    let mut total = 0;
    let (start, map) = parse_input(input);
    let path = get_path(start, &map);

    let height = map.len() as i32;
    let width = if map.len() > 0 { map[0].len() } else { 0 } as i32;
    
    for obstical in path {
        if obstical == start {
            continue;
        }

        let mut visited = HashSet::new();
        let mut v = Vector {
            pos: start,
            dir: (0, -1),
        };

        loop {
            if visited.contains(&v) {
                total += 1;
                break;
            }

            visited.insert(v);

            v.pos.0 += v.dir.0;
            v.pos.1 += v.dir.1;

            if !in_bounds(v.pos, width, height) {
                break;
            }

            if map[v.pos.1 as usize][v.pos.0 as usize] || v.pos == obstical {
                v.pos.0 -= v.dir.0;
                v.pos.1 -= v.dir.1;
                v.dir = rotate(v.dir);
                continue;
            }
        }
    }

    total
}

#[cfg(test)]
mod test {
    use super::*;
    use utils::test_part;

    // Answer: 1976
    test_part!(part_one, TEST_INPUT, 41);
    test_part!(part_two, TEST_INPUT, 6);
}
