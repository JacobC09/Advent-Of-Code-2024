use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

#[allow(dead_code)]
const TEST_INPUT: &str = r#"
125 17
"#;

fn main() {
    println!("Day 11");
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

fn num_digits(num: u64) -> u32 {
    (num as f64).log10().floor() as u32 + 1
}

fn part_one(input: &str) -> u64 {
    let mut stones: Vec<u64> = input
        .split_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect();

    for _ in 0..25 {
        for i in 0..stones.len() {
            let num = stones[i];

            if num == 0 {
                stones[i] = 1;
                continue;
            }

            let digits = num_digits(num);
            if digits % 2 == 0 {
                let mid = digits / 2;
                let divisor = 10u64.pow(mid);
                let left = num / divisor;
                let right = num % divisor;

                stones[i] = left;
                stones.push(right);
                continue;
            }

            stones[i] *= 2024;
        }
    }

    stones.len() as u64
}

fn count(stone: usize, blinks: usize, memory: &mut HashMap<(usize, usize), usize>) -> usize {
    if blinks == 0 {
        return 1;
    }

    if let Some(&cached) = memory.get(&(stone, blinks)) {
        return cached;
    }

    let val = if stone == 0 {
        count(1, blinks - 1, memory)
    } else {
        let num = stone as u64;
        let digits = num_digits(num);
        if digits % 2 == 0 {
            let mid = digits / 2;
            let divisor = 10u64.pow(mid);
            let left = num / divisor;
            let right = num % divisor;
            count(left as usize, blinks - 1, memory) + count(right as usize, blinks - 1, memory)
        } else {
            count(stone * 2024, blinks - 1, memory)
        }
    };

    memory.insert((stone, blinks), val);
    val
}

fn part_two(input: &str) -> usize {
    let mut memory = HashMap::new();

    input
        .split_whitespace()
        .map(|num| count(num.parse::<usize>().unwrap(), 75, &mut memory))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use utils::test_part;

    test_part!(part_one, TEST_INPUT, 55312);
    test_part!(part_two, TEST_INPUT, 55312);
}
