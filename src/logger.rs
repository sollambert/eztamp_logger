use std::io::Write;
use std::fs::File;
use crate::{LogDestination, util::{level::{Message, MessageLevel}, time::TimeStamp}};

#[derive(Debug)]
pub(crate) struct Logger {
    pub(crate) file: Option<File>,
    pub(crate) destination: u8,
    pub(crate) log_level: u16
}

impl Logger {
    pub(crate) fn is_default(&self) -> bool  {
        return self.destination & LogDestination::Default.bits() != 0;
    }

    pub(crate) fn is_std_err(&self) -> bool  {
        return self.destination & LogDestination::StdErr.bits() != 0;
    }

    pub(crate) fn is_std_out(&self) -> bool  {
        return self.destination & LogDestination::StdOut.bits() != 0;
    }

    pub(crate) fn file_logging(&self) -> bool  {
        return self.destination & LogDestination::File.bits() != 0;
    }

    pub(crate) fn log(&self, message: Message) {
        let prefix: &str = message.prefix();
        let level = message.0;
        let msg = message.1.clone();
        let timestamp = TimeStamp::now();

        if !level.is_suppressed(self.log_level) {
            match level {
                MessageLevel::FATAL => {
                    if self.is_default() || self.is_std_err() {
                        eprintln!("[{}][{}]: {}", timestamp.as_utc(), prefix, msg);
                    }
                    if self.is_std_out() {
                        println!("[{}][{}]: {}", timestamp.as_utc(), prefix, msg);
                    }
                },
                MessageLevel::ERROR => {
                    if self.is_default() || self.is_std_err() {
                        eprintln!("[{}][{}]: {}", timestamp.as_utc(), prefix, msg);
                    }
                    if self.is_std_out() {
                        println!("[{}][{}]: {}", timestamp.as_utc(), prefix, msg);
                    }
                },
                MessageLevel::WARN => {
                    if self.is_std_err() {
                        eprintln!("[{}][{}]: {}", timestamp.as_utc(), prefix, msg);
                    }
                    if self.is_default() || self.is_std_out() {
                        println!("[{}][{}]: {}", timestamp.as_utc(), prefix, msg);
                    }
                },
                MessageLevel::INFO => {
                    if self.is_std_err() {
                        eprintln!("[{}][{}]: {}", timestamp.as_utc(), prefix, msg);
                    }
                    if self.is_default() || self.is_std_out() {
                        println!("[{}][{}]: {}", timestamp.as_utc(), prefix, msg);
                    }
                },
                MessageLevel::DEBUG => {
                    if self.is_std_err() {
                        eprintln!("[{}][{}]: {}", timestamp.as_utc(), prefix, msg);
                    }
                    if self.is_default() || self.is_std_out() {
                        println!("[{}][{}]: {}", timestamp.as_utc(), prefix, msg);
                    }
                },
                MessageLevel::TRACE => {
                    if self.is_std_err() {
                        eprintln!("[{}][{}]: {}", timestamp.as_utc(), prefix, msg);
                    }
                    if self.is_default() || self.is_std_out() {
                        println!("[{}][{}]: {}", timestamp.as_utc(), prefix, msg);
                    }
                },
                MessageLevel::CUSTOM(v) => {
                    if self.is_std_err() {
                        eprintln!("[{}][{}({})]: {}", timestamp.as_utc(), prefix, v, msg);
                    }
                    if self.is_default() || self.is_std_out() {
                        println!("[{}][{}({})]: {}", timestamp.as_utc(), prefix, v, msg);
                    }
                }
            }

            if self.file_logging() {
                let prefix = match level {
                    MessageLevel::FATAL => "FATAL",
                    MessageLevel::ERROR => "ERROR",
                    MessageLevel::WARN => "WARN",
                    MessageLevel::INFO => "INFO",
                    MessageLevel::DEBUG => "DEBUG",
                    MessageLevel::TRACE => "TRACE",
                    MessageLevel::CUSTOM(_) => message.2.unwrap_or("CUSTOM"),
                };
                if let Some(file) = &self.file {
                    let mut file = file;
                    let _ = writeln!(file, "{};{};{}", timestamp.as_utc(), prefix, msg);
                }
            }
        }
    }
}