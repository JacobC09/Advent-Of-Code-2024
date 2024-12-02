const INPUT: &str = include_str!("input.txt");

#[allow(dead_code)]
const TEST_INPUT: &str = r#"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;

fn main() {
    println!("Day 2:");
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}


fn get_reports(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())  
                .collect()
        })
        .collect()
}

fn get_error(report: &Vec<i32>) -> Option<i32> {
    let mut factor = 0;

    for i in 1..report.len() {
        let dif = report[i] - report[i - 1] as i32;

        if factor == 0 {
            factor = dif.signum();
        }

        if dif == 0 || dif.abs() > 3 || dif * factor < 0 {
            return Some(i as i32);
        }
    }

    None
}

fn part_one(input: &str) -> i32 {
    let mut total = 0;
    let reports = get_reports(input);

    for report in reports {
        if get_error(&report).is_none() {
            total += 1;
        }
    }

    total
}

fn part_two(input: &str) -> i32 {
    let mut total = 0;
    let reports = get_reports(input);

    'outer: for report in reports {
        if get_error(&report).is_none() {
            total += 1;
            continue;
        }

        for i in 0..report.len() {
            let mut new = report.clone();
            new.remove(i);

            if get_error(&new).is_none() {
                total += 1;
                continue 'outer;
            }
        }
    }

    total
}

#[cfg(test)]
mod test {
    use super::*;
    use utils::test_part;

    test_part!(part_one, TEST_INPUT, 2);
    test_part!(part_two, TEST_INPUT, 4);
}
