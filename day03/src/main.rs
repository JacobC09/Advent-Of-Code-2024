const INPUT: &str = include_str!("input.txt");

#[allow(dead_code)]
const TEST_INPUT_ONE: &str = r#"
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
"#;

#[allow(dead_code)]
const TEST_INPUT_TWO: &str = r#"
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
"#;

fn main() {
    println!("Day 3:");
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

fn parse_num(input: &str) -> (i32, usize) {
    for (index, ch) in input.chars().enumerate() {
        if !ch.is_numeric() {
            let mut num = 0;

            if let Ok(parsed) = input[..index].parse::<i32>() {
                num = parsed;
            }

            return (num, index);
        }
    }

    (0, 0)
}

fn parse_mul(input: &str) -> (usize, i32) {
    if input.starts_with("mul(") {
        let mut i = 4;

        let (num1, offset) = parse_num(&input[i..]);

        if offset == 0 {
            return (i, 0);
        }

        i += offset;
        if !input[i..].starts_with(",") {
            return (i, 0);
        }

        i += 1;
        let (num2, offset) = parse_num(&input[i..]);
        if offset == 0 {
            return (i, 0);
        }

        i += offset;
        if !input[i..].starts_with(")")  {
            return (i, 0);
        }

        return (i + 1, num1 * num2);
    }

    (1, 0)
}

fn part_one(input: &str) -> i32 {
    let mut total = 0;
    let mut i = 0;
    
    while i < input.len() {
        let (offset, value) = parse_mul(&input[i..]);
        total += value;
        i += offset;
    }

    total
}

fn part_two(input: &str) -> i32 {
    let mut total = 0;
    let mut i = 0;
    let mut enabled = true;

    while i < input.len() {
        if input[i..].starts_with("do()") {
            i += 4;
            enabled = true;
            continue;
        }

        if input[i..].starts_with("don't()") {
            i += 7;
            enabled = false;
            continue;
        }

        if enabled {
            let (offset, value) = parse_mul(&input[i..]);
            total += value;
            i += offset;
        } else {
            i += 1;
        }
    }

    total
}

#[cfg(test)]
mod test {
    use super::*;
    use utils::test_part;

    test_part!(part_one, TEST_INPUT_ONE, 161);
    test_part!(part_two, TEST_INPUT_TWO, 48);
}
