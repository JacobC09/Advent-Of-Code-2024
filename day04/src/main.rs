const INPUT: &str = include_str!("input.txt");

#[allow(dead_code)]
const TEST_INPUT: &str = r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;

fn main() {
    println!("Day 4:");
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}


fn part_one(input: &str) -> i32 {
    const WORD: &str = "XMAS";
    let mut total = 0;
    let word_len = WORD.len();
    let grid: Vec<&[u8]> = input.lines()
        .filter(|line| line.len() > 0)
        .map(|line| line.as_bytes())
        .collect();
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };
    let target = WORD.as_bytes();
    let target_reversed: Vec<u8> = WORD.bytes().rev().collect();


    for row in &grid {
        total += row.windows(word_len)
            .filter(|&window| window == target || window == target_reversed)
            .count() as i32;
    }

    
    for col in 0..cols {
        let mut column = vec![];
        for row in 0..rows {
            column.push(grid[row][col]);
        }

        total += column.windows(word_len)
            .filter(|&window| window == target || window == target_reversed)
            .count() as i32;
    }

    for y in 0..rows - word_len + 1 {
        for x in 0..(cols - word_len + 1) {
            let mut slice = vec![];
            for offset in 0..word_len {
                slice.push(grid[y + offset][x + offset]);
            }

            if slice == target || slice == target_reversed {
                total += 1
            }
        }
    }

    for y in word_len - 1..rows {
        for x in 0..(cols - word_len + 1) {
            let mut slice = vec![];
            for offset in 0..word_len {
                slice.push(grid[y - offset][x + offset]);
            }

            if slice == target || slice == target_reversed {
                total += 1
            }
        }
    }

    total
}

fn part_two(input: &str) -> i32 {
    let mut total = 0;
    let grid: Vec<&[u8]> = input.lines()
        .filter(|line| line.len() > 0)
        .map(|line| line.as_bytes())
        .collect();
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };
    let window_size = 3;

    for y in 0..rows - window_size + 1 {
        'outer: for x in 0..cols - window_size + 1 {
            if grid[y + 1][x + 1] != b'A' {
                continue;
            }

            for k in (0..window_size).step_by(2) {
                for j in (0..window_size).step_by(2) {
                    let ch = grid[y + k][x + j];
                    if ch != b'M' && ch != b'S' {
                        continue 'outer;
                    }
                }
            }

            if grid[y][x] != grid[y + 2][x + 2] && grid[y][x + 2] != grid[y + 2][x] {
                total += 1;
            }
        }
    }

    total
}

#[cfg(test)]
mod test {
    use super::*;
    use utils::test_part;

    test_part!(part_one, TEST_INPUT, 18);
    test_part!(part_two, TEST_INPUT, 9);
}
