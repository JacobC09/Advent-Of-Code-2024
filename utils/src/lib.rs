#[macro_export]
macro_rules! test_part {
    ($func:ident, $input:expr, $expected:expr) => {
        #[test]
        fn $func() {
            let result = super::$func($input);

            if result != $expected {
                println!();
                println!("{} -> {}", format!("\x1b[1;31m{}\x1b[0;0m", "Test Failed:"), stringify!($func));
                println!("  Expected: {}", format!("\x1b[0;32m{}\x1b[0;0m", format!("{}", $expected)));
                println!("  Got: {}", format!("\x1b[0;31m{}\x1b[0;0m", format!("{}", result)));
                println!();
                panic!();
            }
        }
    };
}

/*

// Template
const INPUT: &str = include_str!("input.txt");

#[allow(dead_code)]
const TEST_INPUT: &str = r#"

"#;

fn main() {
    println!("Day 5:");
    println!("Part one: {}", part_one(INPUT));
    // println!("Part two: {}", part_two(INPUT));
}

fn part_one(input: &str) -> i32 {
    0
}

// fn part_two(input: &str) -> i32 {
//     0
// }

#[cfg(test)]
mod test {
    use super::*;
    use utils::test_part;

    test_part!(part_one, TEST_INPUT, 0);
    // test_part!(part_two, TEST_INPUT, 0);
}

*/