use std::io::Write;
use std::fs::File;

use crate::LogDestination;
use crate::message::Message;
use crate::message::level::MessageLevel;

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
        let level = message.level();

        if !level.is_suppressed(self.log_level) {
            match level {
                MessageLevel::FATAL | MessageLevel::ERROR => {
                    if self.is_default() || self.is_std_err() {
                        eprintln!("{}", message.to_display_string());
                    }
                    if self.is_std_out() {
                        println!("{}", message.to_display_string());
                    }
                },
                MessageLevel::WARN | MessageLevel::INFO | MessageLevel::DEBUG | MessageLevel::TRACE | MessageLevel::CUSTOM(_) => {
                    if self.is_std_err() {
                        eprintln!("{}", message.to_display_string());
                    }
                    if self.is_default() || self.is_std_out() {
                        println!("{}", message.to_display_string());
                    }
                }
            }

            if self.file_logging() {
                if let Some(file) = &self.file {
                    let mut file = file;
                    let _ = writeln!(file, "{}", message.to_string());
                }
            }
        }
    }
}