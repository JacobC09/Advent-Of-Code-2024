use std::{collections::HashMap, fmt::format};

const INPUT: &str = include_str!("input.txt");

#[allow(dead_code)]
const TEST_INPUT: &str = r#"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;

fn main() {
    println!("Day 5:");
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

#[derive(Debug)]
struct Test {
    target: u64,
    values: Vec<u64>,
}

fn get_tests(input: &str) -> Vec<Test> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (left, right) = line.split_once(":").unwrap();
            let target = left
                .parse::<u64>()
                .unwrap();
            let values = right
                .split_whitespace()
                .map(|s| s.parse::<u64>().unwrap()) 
                .collect();

            Test { target, values }
        })
        .collect()
}

fn part_one(input: &str) -> u64 {
    let mut sum = 0;
    let tests = get_tests(input);

    for test in tests {
        for combo in 0..1 << (test.values.len() - 1) {
            let mut total = test.values[0];

            for (i, val) in test.values[1..].iter().enumerate() {
                if combo & (1 << i) > 0 {
                    total *= val;
                } else {
                    total += val;
                }
            }

            if total == test.target {
                sum += test.target;
                break;
            }
            
        }
    }

    sum
}

fn can_eval(current: u64, nums: &[u64], target: u64) -> bool {
    if current > target {
        return false;
    }

    if nums.is_empty() {
        return target == current;
    }

    let next = nums[0];
    let remaining = &nums[1..];

    can_eval(current + next, remaining, target) ||
    can_eval(current * next, remaining, target) ||
    can_eval(format!("{}{}", current, next).parse().unwrap(), remaining, target)
}

fn part_two(input: &str) -> u64 {
    let mut sum = 0;
    let tests = get_tests(input);

    for test in tests {
        if can_eval(test.values[0], &test.values[1..], test.target) {
            sum += test.target;
        }
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;
    use utils::test_part;

    test_part!(part_one, TEST_INPUT, 3749);
    test_part!(part_two, TEST_INPUT, 11387);
}
