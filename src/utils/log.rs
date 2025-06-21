#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        #[cfg(feature = "debug")]
        {
            println!($($arg)*);
        }
    };
}
