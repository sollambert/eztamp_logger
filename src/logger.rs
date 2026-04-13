use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::str::Lines;

use sha2::digest::Output;
use sha2::{Digest, Sha256};
use hmac::{Hmac, KeyInit, Mac};

type Hmac256 = Hmac<Sha256>;

use crate::LogDestination;
use crate::message::Message;
use crate::message::level::MessageLevel;
use crate::util::{Hex};

#[derive(Debug)]
pub(crate) struct Logger {
    pub(crate) file: Option<File>,
    pub(crate) destination: u8,
    pub(crate) log_level: u16,
    pub(crate) secret: String
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
                let file = self.file.as_mut().unwrap();
                let _lock = file.lock();
                let last_line = match read_last_line(&file) {
                    Ok(line) => line,
                    Err(_) => String::new(),
                };

                // get secret
                let secret = &self.secret;
                let secret_str = secret.as_str();

                // extract prev mac
                let parts: Vec<&str> = last_line.splitn(5, ';').collect();
                let mut prev_key_hex = parts.get(0).copied().unwrap_or(&secret_str);
                if prev_key_hex == "" {prev_key_hex = &secret_str};
                let prev_mac_hex = parts.get(1).copied().unwrap_or(&"");

                // decode from hex
                let prev_key = Hex::decode(prev_key_hex).unwrap();
                let prev_mac = Hex::decode(prev_mac_hex).unwrap();
                
                // generate new mac
                let mut hasher = Sha256::new();
                hasher.update(prev_key);
                let k_i = hasher.finalize();
                let new_key_hex = Hex::encode(k_i.bytes());

                // hash message
                let mut mac = Hmac256::new_from_slice(&k_i).unwrap();
                update_field(&mut mac, &prev_mac);
                update_field(&mut mac, message.timestamp().as_utc().as_bytes());
                update_field(&mut mac, &message.level().as_u16().to_be_bytes());
                update_field(&mut mac, message.text().as_bytes());
                let mac_bytes = mac.finalize().into_bytes();
                let mac_hex = Hex::encode(mac_bytes.bytes());

                // write to file
                let _ = writeln!(file, "{};{};{}", new_key_hex, mac_hex, message.to_string());
            }
        }
    }
}

pub(crate) fn verify_log(mut file: File, initial_key: String) -> Result<(), String> {
    let mut prev_key = initial_key;
    let mut prev_mac = Hex::decode("").unwrap();
    let mut buf = String::new();
    let _ = file.read_to_string(&mut buf);

    let mut line_index = 0;
    for line in buf.lines() {
        let parts: Vec<&str> = line.splitn(5, ';').collect();
        if parts.len() != 5 {
            return Err(format!("invalid format at line {}", line_index));
        }

        let k_i = parts[0];
        let k_i_bytes = Hex::decode(k_i)?;
        let mac_i = parts[1];
        let mac_i_bytes = Hex::decode(mac_i)?;
        let timestamp = parts[2];
        let level = parts[3];
        let message = parts[4];
        // 1. verify key chain
        let mut hasher = Sha256::new();
        hasher.update(Hex::decode(&prev_key).unwrap());
        let expected_k_i = hasher.finalize();
        let expected_k_i_hex = Hex::encode(expected_k_i.bytes());
        if k_i != expected_k_i_hex {
            return Err(format!("key chain broken at line {} | actual: {} | expected: {}", line_index, k_i, expected_k_i_hex));
        }

        // 2. verify MAC
        let mut mac = Hmac256::new_from_slice(&expected_k_i).unwrap();

        update_field(&mut mac, &prev_mac);
        update_field(&mut mac, timestamp.as_bytes());
        update_field(&mut mac, &MessageLevel::from(level).as_u16().to_be_bytes());
        update_field(&mut mac, message.as_bytes());
        
        let expected_mac_bytes = mac.finalize().into_bytes();
        let expected_mac_hex = Hex::encode(expected_mac_bytes.bytes());

        if mac_i != expected_mac_hex {
            return Err(format!("mac invalid at line {} | actual: {} | expected: {}", line_index, mac_i, expected_mac_hex));
        }

        // 3. advance state
        prev_key = Hex::encode(k_i_bytes.bytes());
        prev_mac = mac_i_bytes;
        line_index += 1;
    }

    Ok(())
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

fn update_field(mac: &mut Hmac256, data: &[u8]) {
    mac.update(&(data.len() as u64).to_be_bytes());
    mac.update(data);
}