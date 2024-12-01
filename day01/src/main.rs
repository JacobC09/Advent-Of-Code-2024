const INPUT: &str = include_str!("input.txt");

#[allow(dead_code)]
const TEST_INPUT: &str = r#"
3   4
4   3
2   5
1   3
3   9
3   3
"#;

fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

fn parse_numbers(string: &str) -> Vec<i32> {
    let mut numbers = Vec::new();
    let mut cur = String::new();

    for ch in string.chars() {
        if ch.is_numeric() {
            cur.push(ch);
        } else if !cur.is_empty() {
            if let Ok(number) = cur.parse::<i32>() {
                numbers.push(number);
            }
            cur.clear();
        }
    }


    if !cur.is_empty() {
        if let Ok(number) = cur.parse::<i32>() {
            numbers.push(number);
        }
    }

    numbers
}

fn part_one(input: &str) -> i32 {
    let numbers = parse_numbers(input);
    let mut total = 0;
    let mut left = Vec::new();
    let mut right = Vec::new();

    for i in (0..numbers.len()).step_by(2) {
        left.push(numbers[i]);
        right.push(numbers[i + 1]);
    }

    left.sort();
    right.sort();

    for (a, b) in left.iter().zip(right.iter()) {
        total += (a - b).abs();
    }

    total
}

fn part_two(input: &str) -> i32 {
    let numbers = parse_numbers(input);
    let mut score = 0;
    let mut left = Vec::new();
    let mut right = Vec::new();

    for i in (0..numbers.len()).step_by(2) {
        left.push(numbers[i]);
        right.push(numbers[i + 1]);
    }

    for val in left {
        score += right.iter()
            .filter(|&&v| v == val)
            .count() as i32 * val
    }

    score
}

#[cfg(test)]
mod test {
    use super::*;
    use utils::test_part;

    test_part!(test_part_one, part_one, TEST_INPUT, 11);
    test_part!(test_part_two, part_two, TEST_INPUT, 31);
}
