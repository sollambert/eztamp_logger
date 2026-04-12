use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

use sha2::{Digest, Sha256};

use crate::LogDestination;
use crate::message::Message;
use crate::message::level::MessageLevel;

#[derive(Debug)]
pub(crate) struct Logger {
    pub(crate) file: Option<File>,
    pub(crate) destination: u8,
    pub(crate) log_level: u16,
    pub(crate) salt: String
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

    pub(crate) fn log(&mut self, message: Message) {
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
                let message_string = message.to_string();
                let file = self.file.as_mut().unwrap();
                let _lock = file.lock();
                let last_line = match read_last_line(&file) {
                    Ok(line) => line,
                    Err(_) => String::new(),
                };
                let mut buff = Vec::<char>::new();
                for new_char in last_line.chars() {
                    if new_char == ';' {break;} else {buff.push(new_char)}
                }
                let prev_checksum = buff.iter().collect::<String>();
                let mut hasher = Sha256::new();
                hasher.update(&self.salt);
                hasher.update(prev_checksum.as_bytes());
                hasher.update(message_string.clone());
                let digest = hasher.finalize();
                let hex_string: String = digest
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect();
                let _ = writeln!(file, "{};{}", hex_string, message_string);
            }
        }
    }
}

fn read_last_line(mut file: &File) -> Result<String, std::io::Error> {
    let mut pos = file.seek(SeekFrom::End(0))?;
    let mut buf = Vec::new();

    while pos > 0 {
        pos -= 1;
        file.seek(SeekFrom::Start(pos))?;

        let mut byte = [0];
        file.read_exact(&mut byte)?;

        if byte[0] == b'\n' && !buf.is_empty() {
            break;
        }

        buf.push(byte[0]);
    }

    buf.reverse();
    Ok(String::from_utf8_lossy(&buf).to_string())
}