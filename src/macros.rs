use core::fmt;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::macros::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    super::screen::WRITER.lock().write_fmt(args).unwrap();
}

impl fmt::Write for super::screen::Screen {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
