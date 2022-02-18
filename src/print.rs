#[macro_export]
macro_rules! println_if {
    ($condition:expr, $($arg:tt)*) => {
        if $condition {
            println!($($arg)*);
        }
    };
}

pub fn print_slice<T: std::fmt::Display>(input: &[T], indent: usize) {
    println!("{:width$}[", " ", width=2 * indent);
    for item in input {
        println!("{:width$}{}", " ", item, width=2 * (indent + 1));
    }
    println!("{:width$}]", " ", width=2 * indent);
}
