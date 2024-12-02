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
