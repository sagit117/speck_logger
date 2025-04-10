#![crate_type = "lib"]
#![crate_name = "speck_logger"]

use std::{fmt, io::Write};

#[derive(PartialEq, PartialOrd)]
pub enum LogLevel {
    ALL = 0,
    DEBUG = 10,
    INFO = 20,
    WARNING = 30,
    ERROR = 40,
}

impl LogLevel {
    /// Метод воссоздаст Level из строки или вернет Level::ALL, если операция не будет успешной.
    pub fn from_string(str: &str) -> LogLevel {
        let level = str.to_uppercase();

        if level.eq("DEBUG") {
            LogLevel::DEBUG
        } else if level.eq("ONFO") {
            LogLevel::INFO
        } else if level.eq("WARNING") {
            LogLevel::WARNING
        } else if level.eq("ERROR") {
            LogLevel::ERROR
        } else {
            LogLevel::ALL
        }
    }
}

/// Логгер.
/// 
/// В качестве источника вывода лога можно применить файл или другую цель реализующую трейт std::io::Write
/// 
/// # Example
/// 
/// ```
/// use crate::speck_logger::LogWriter;
/// let f = std::io::stdout();
/// 
/// let mut logger = speck_logger::Logger {
///     out: Box::new(f),
///     level: speck_logger::LogLevel::ALL,
///     formatter: |message: &str, level: speck_logger::LogLevel| -> String { format!("[{: <7}] {}\n", level.to_string(), message) }
/// };
/// 
/// logger.info("Информация");
/// logger.error("Ошибка");
/// ```
pub struct Logger {
    pub out: Box<dyn Write>,
    pub level: LogLevel,
    pub formatter: fn (&str, LogLevel) -> String
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::DEBUG => fmt::write(f, format_args!("DEBUG")),
            LogLevel::INFO => fmt::write(f, format_args!("INFO")),
            LogLevel::WARNING => fmt::write(f, format_args!("WARNING")),
            LogLevel::ERROR => fmt::write(f, format_args!("ERROR")),
            LogLevel::ALL => fmt::write(f, format_args!("ALL")),
        }
    }
}

impl Logger {
    fn log(&mut self, message: &str, level: LogLevel) -> Result<(), Box<dyn std::error::Error>> {
        if self.level <= level {
            let format_string = (self.formatter)(message, level);
            self.out.write(format_string.as_bytes())?;
        }

        Ok(())
    }
}

pub type LogResult = Result<(), Box<(dyn std::error::Error + 'static)>>;

impl LogWriter<LogResult> for Logger {
    fn debug(&mut self, message: &str) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
        self.log(message, LogLevel::DEBUG)
    }

    fn info(&mut self, message: &str) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
        self.log(message, LogLevel::INFO)
    }

    fn warning(&mut self, message: &str) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
        self.log(message, LogLevel::WARNING)
    }

    fn error(&mut self, message: &str) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
        self.log(message, LogLevel::ERROR)
    }
}

pub trait LogWriter<T> {
    fn info(&mut self, message: &str) -> T;
    fn warning(&mut self, message: &str) -> T;
    fn debug(&mut self, message: &str) -> T;
    fn error(&mut self, message: &str) -> T;
}
