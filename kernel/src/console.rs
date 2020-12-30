use crate::sbi::console_putchar;
use core::fmt::{self, Write};
use lazy_static::*;
use log::{self, Level, LevelFilter, Log, Metadata, Record};
use spin::Mutex;

struct Stdout;

pub fn init(level: &str) {
    static LOGGER: SimpleLogger = SimpleLogger;
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(match level {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Off,
    });
    log::set_max_level(LevelFilter::Info);
}

lazy_static! {
    static ref STDOUT: Mutex<Stdout> = Mutex::new(Stdout);
}

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    STDOUT.lock().write_fmt(args).unwrap();
}

macro_rules! with_color {
    ($args: ident, $color_code: ident) => {{
        format_args!("\u{1B}[{}m{}\u{1B}[0m", $color_code as u8, $args)
    }};
}

pub fn print_in_color(args: fmt::Arguments, color_code: u8) {
    STDOUT
        .lock()
        .write_fmt(with_color!(args, color_code))
        .unwrap();
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

fn level_to_color_code(level: Level) -> u8 {
    match level {
        Level::Error => 31, // Red
        Level::Warn => 93,  // BrightYellow
        Level::Info => 34,  // Blue
        Level::Debug => 32, // Green
        Level::Trace => 90, // BrightBlack
    }
}

struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn flush(&self) {}

    fn log(&self, record: &Record) {
        print_in_color(
            format_args!(
                "[{:>5}][{}] {}\n",
                record.level(),
                crate::cpu::id(),
                record.args()
            ),
            level_to_color_code(record.level()),
        );
    }
}
