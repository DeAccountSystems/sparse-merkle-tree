#[macro_export]
macro_rules! println_if {
    ($condition:expr, $($arg:tt)*) => {
        if $condition {
            println!($($arg)*);
        }
    };
}
