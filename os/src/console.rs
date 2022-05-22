use core::fmt::{self, Write};

use crate::sbi::console_put_char;

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            console_put_char(b as usize);
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

pub struct LevelConfig {
    pub color_code: u8,
    pub prefix: &'static str,
}

impl Level {
    pub fn get_config(&self) -> LevelConfig {
        match self {
            Level::Error => LevelConfig {
                color_code: 31,
                prefix: "💥 ",
            },
            Level::Warn => LevelConfig {
                color_code: 93,
                prefix: "✨ ",
            },
            Level::Info => LevelConfig {
                color_code: 34,
                prefix: "📎 ",
            },
            Level::Debug => LevelConfig {
                color_code: 32,
                prefix: "🐛 ",
            },
            Level::Trace => LevelConfig {
                color_code: 90,
                prefix: "🐾 ",
            },
        }
    }
}

macro_rules! log {
    ($lvl: expr, $fmt: literal $(, $($arg: tt)+)?) => {
        let config = $lvl.get_config();
        $crate::console::print_in_color(
            format_args!(
                concat!("{}", $fmt, "\n"),
                config.prefix $(, $($arg)+)?
            ),
            config.color_code
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
