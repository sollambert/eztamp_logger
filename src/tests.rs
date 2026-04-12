#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{debug, error, fatal, info, log, message::{Message, MessagePrefix, level::MessageLevel}, trace, warn};

    #[test]
    fn all() {
        let _ = dotenv::dotenv();
        crate::init();
        fatal!("This is a fatal error!");
        error!("This is an error!");
        warn!("This is a warn!");
        info!("This is an info message!");
        debug!("This is a debug message!");
        trace!("This is a trace message!");
        std::thread::sleep(Duration::new(0, 10_000_000));
    }

    #[test]
    fn custom() {
        let _ = dotenv::dotenv();
        crate::init();
        let message = Message::new(MessageLevel::CUSTOM(320), "This is a custom message!".to_string(), MessagePrefix::custom_prefix("TEST"));
        log!(message);
        std::thread::sleep(Duration::new(0, 10_000_000));
    }
}

#[cfg(test)]
mod validator {
    use std::{env, fs::OpenOptions, io::Read};
    use sha2::{Digest, Sha256};
    
    #[test]
    fn validate() {
        let _ = dotenv::dotenv();
        let salt = env::var("RUST_LOG_SALT").unwrap_or_default();
        let output_path = env::var("RUST_LOG_OUTPUT_FILE").unwrap_or("./log.txt".to_string());
        let mut file = OpenOptions::new()
            .read(true)
            .open(output_path).unwrap();
        let mut content = String::new();
        let _ = file.read_to_string(&mut content);
        let mut line_index = 0;
        for line in content.lines() {
            let mut hasher = Sha256::new();
            let mut parts = line.split(";");
            let checksum = parts.next().unwrap();
            let timestamp = parts.next().unwrap();
            let prefix = parts.next().unwrap();
            let message = parts.next().unwrap();
            if line_index != 0 {
                let prev_message = content.lines().nth(line_index - 1).unwrap();
                let mut pref_checksum_buf = Vec::<char>::new();
                for new_char in prev_message.chars() {
                    if new_char == ';' {
                        break;
                    }
                    pref_checksum_buf.push(new_char);
                }
                let prev_checksum = pref_checksum_buf.iter().collect::<String>();
                hasher.update(&salt);
                hasher.update(prev_checksum);
                hasher.update(format!("{};{};{}", timestamp, prefix, message));
                let digest = hasher.finalize();
                let hex_string: String = digest
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect();
                println!("({}) | {} | {}", line_index + 1, checksum, hex_string);
                assert_eq!(checksum, hex_string);
            } else {
                hasher.update(&salt);
                hasher.update(format!("{};{};{}", timestamp, prefix, message));
                let digest = hasher.finalize();
                let hex_string: String = digest
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect();
                println!("({}) | {} | {}", line_index + 1, checksum, hex_string);
                assert_eq!(checksum, hex_string);
            }
            line_index += 1;
        }
        println!("Log validated, no checksum mismatch detected")
    }
}