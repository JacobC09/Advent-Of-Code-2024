use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

#[allow(dead_code)]
const TEST_INPUT: &str = r#"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;

fn main() {
    println!("Day 5:");
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

fn parse_input(input: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let mut lines = input.lines();
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();

    for line in &mut lines {
        if line.is_empty() {
            if rules.len() == 0 {
                continue;
            }
            break;
        }

        let nums: Vec<i32> = line
            .split("|")
            .map(|num| num.parse::<i32>().unwrap())
            .collect();

        rules.entry(nums[0]).or_insert_with(Vec::new).push(nums[1]);
    }

    let updates: Vec<Vec<i32>> = (&mut lines)
        .map(|line| {
            line.split(",")
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    (rules, updates)
}

fn part_one(input: &str) -> i32 {
    let mut middle_pages = 0;
    let (rules, updates) = parse_input(input);

    for update in updates {
        let mut valid = true;

        for (i, num) in update[..update.len()].iter().enumerate() {
            if let Some(rule) = rules.get(num) {
                for right in update[i + 1..].iter() {
                    if !rule.contains(right) {
                        valid = false;
                        break;
                    }
                }

                for left in update[..i].iter() {
                    if rule.contains(left) {
                        valid = false;
                        break;
                    }
                }
            }
        }

        if valid {
            middle_pages += update[(update.len() - 1) / 2];
        }
    }

    middle_pages
}

fn part_two(input: &str) -> i32 {
    let mut middle_pages = 0;
    let (rules, updates) = parse_input(input);

    for update in updates {
        let mut copy = update.clone();
        copy.sort_by(|left, right| {
            if let Some(rule) = rules.get(&left) {
                if rule.contains(right) {
                    return std::cmp::Ordering::Less;
                }
            }

            if let Some(rule) = rules.get(&right) {
                if rule.contains(left) {
                    return std::cmp::Ordering::Greater;
                }
            }
            
            std::cmp::Ordering::Equal
        });

        if update != copy {
            middle_pages += copy[(update.len() - 1) / 2];
        }
    }

    middle_pages
}

#[cfg(test)]
mod test {
    use super::*;
    use utils::test_part;

    test_part!(part_one, TEST_INPUT, 143);
    test_part!(part_two, TEST_INPUT, 123);
}
