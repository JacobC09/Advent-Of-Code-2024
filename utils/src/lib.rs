#[macro_export]
macro_rules! test_part {
    ($test_name:ident, $func:ident, $input:expr, $expected:expr) => {
        #[test]
        fn $test_name() {
            let result = $func($input);

            if result != $expected {
                println!();
                println!("{} -> {}", format!("\x1b[1;31m{}\x1b[0;0m", "Test Failed:"), stringify!($func));
                println!("  {} {}", format!("\x1b[0;32m{}\x1b[0;0m", "Expected:"), format!("{}", $expected));
                println!("  {} {}", format!("\x1b[0;31m{}\x1b[0;0m", "Got:"), format!("{}", result));
                println!();
            }
            assert_eq!(result, $expected);
        }
    };
}
