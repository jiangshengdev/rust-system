use core::fmt::{self, Write};

use crate::sbi::console_putchar;

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            console_putchar(b as usize);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

macro_rules! with_color {
    ($args: ident, $color_code: ident) => {
        format_args!("\u{1B}[{}m{}\u{1B}[0m", $color_code, $args)
    };
}

pub fn print_in_color(args: fmt::Arguments, color_code: u8) {
    Stdout.write_fmt(with_color!(args, color_code)).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

pub enum Level {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

pub struct LevelStyle {
    pub color_code: u8,
    pub prefix: &'static str,
}

impl Level {
    pub fn get_style(&self) -> LevelStyle {
        match self {
            Level::Error => LevelStyle {
                color_code: 31,
                prefix: "💥 ",
            },
            Level::Warn => LevelStyle {
                color_code: 93,
                prefix: "✨ ",
            },
            Level::Info => LevelStyle {
                color_code: 34,
                prefix: "📎 ",
            },
            Level::Debug => LevelStyle {
                color_code: 32,
                prefix: "🐛 ",
            },
            Level::Trace => LevelStyle {
                color_code: 90,
                prefix: "🐾 ",
            },
        }
    }
}

macro_rules! log {
    ($lvl: expr, $fmt: literal $(, $($arg: tt)+)?) => {
        let style = $lvl.get_style();
        $crate::console::print_in_color(
            format_args!(
                concat!("{}", $fmt, "\n"),
                style.prefix $(, $($arg)+)?
            ),
            style.color_code
        );
    }
}

#[macro_export]
macro_rules! error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        log!($crate::console::Level::Error, $fmt $(, $($arg)+)?);
    }
}

#[macro_export]
macro_rules! warn {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        log!($crate::console::Level::Warn, $fmt $(, $($arg)+)?);
    }
}

#[macro_export]
macro_rules! info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        log!($crate::console::Level::Info, $fmt $(, $($arg)+)?);
    }
}

#[macro_export]
macro_rules! debug {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        log!($crate::console::Level::Debug, $fmt $(, $($arg)+)?);
    }
}

#[macro_export]
macro_rules! trace {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        log!($crate::console::Level::Trace, $fmt $(, $($arg)+)?);
    }
}
