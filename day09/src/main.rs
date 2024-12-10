use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

#[allow(dead_code)]
const TEST_INPUT: &str = "2333133121414131402";

fn main() {
    println!("Day 9");
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

fn part_one(input: &str) -> usize {
    let mut structure = Vec::new();

    for (i, ch) in input.chars().enumerate() {
        let len = ch.to_digit(10).unwrap();
        if i % 2 == 0 {
            let id = i / 2;

            for _ in 0..len {
                structure.push(Some(id));
            }
        } else {
            for _ in 0..len {
                structure.push(None);
            }
        }
    }

    let mut left: usize = 0;
    let mut right: usize = structure.len() - 1;

    while left < right {
        if structure[left] == None {
            while structure[right] == None {
                right -= 1;
            }

            structure[left] = structure[right];
            right -= 1;
        }

        left += 1
    }

    structure
        .iter()
        .take(left + 1)
        .enumerate()
        .fold(0, |checksum, (i, optional)| {
            if let Some(val) = optional {
                checksum + i * val
            } else {
                checksum
            }
        })
}

#[derive(Clone, Copy, Debug)]
struct Block {
    len: u32,
    id: Option<usize>,
}

fn part_two(input: &str) -> usize {
    let mut blocks: Vec<Block> = input
        .chars()
        .enumerate()
        .map(|(i, ch)| {
            let len = ch.to_digit(10).unwrap();
            let id = if i % 2 == 0 { Some(i / 2) } else { None };
            Block { len, id }
        })
        .collect();

    let mut right = blocks.len().saturating_sub(1);
    let mut last_gap = HashMap::new();  // Added for optimization

    while right > 0 {
        let block = blocks[right];

        if block.id.is_none() {
            right -= 1;
            continue;
        }

        let start = last_gap
            .keys()
            .filter(|&&key| key < block.len)
            .max()
            .and_then(|&key| last_gap.get(&key))
            .cloned()
            .unwrap_or(0);

        for left in start..right {
            if let Some(space) = blocks.get_mut(left) {
                if space.id.is_some() {
                    continue;
                }

                if space.len >= block.len {
                    last_gap.insert(space.len, left + 1);
                    space.len -= block.len;
                    blocks[right].id = None;
                    blocks.insert(left, block);
                    break;
                }
            }
        }

        right -= 1;
    }

    let mut checksum = 0;
    let mut i = 0;
    for block in blocks {
        if let Some(id) = block.id {
            for j in i..i + block.len {
                checksum += id * j as usize;
            }
        }
        i += block.len;
    }

    checksum
}

#[cfg(test)]
mod test {
    use super::*;
    use utils::test_part;

    test_part!(part_one, TEST_INPUT, 1928);
    test_part!(part_two, TEST_INPUT, 2858);
}
