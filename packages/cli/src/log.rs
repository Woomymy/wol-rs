#[macro_export]
/// Prints an error (in red)
macro_rules! error {
    ($($arg:tt)*) => {
        println!("{}", format!("\x1B[1;91m{}\x1b[0;m", format!($($arg)*)))
    };
}

#[macro_export]
/// Prints an information
macro_rules! info {
    ($($arg:tt)*) => {
        println!("{}", format!("\x1B[1;96m{}\x1b[0;m", format!($($arg)*)))
    };
}

#[macro_export]
/// Prints an debug info, in yellow
macro_rules! debug {
    ($($arg:tt)*) => {
        println!("{}", format!("\x1B[1;93m[{}:{}:{}] {}\x1b[0;m", file!(), line!(), column!(), format!($($arg)*)))
    };
}
