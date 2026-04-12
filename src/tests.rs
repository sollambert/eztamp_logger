#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{debug, error, fatal, info, log, message::{Message, MessagePrefix, level::MessageLevel}, trace, warn};

    #[test]
    fn all() {
        crate::init();
        fatal!("This is a fatal error!");
        error!("This is an error!");
        warn!("This is a warn!");
        info!("This is an info message!");
        debug!("This is a debug message!");
        trace!("This is a trace message!");
        std::thread::sleep(Duration::new(0, 250000));
    }

    #[test]
    fn custom() {
        crate::init();
        let message = Message::new(MessageLevel::CUSTOM(320), "This is a custom message!".to_string(), MessagePrefix::custom_prefix("TEST"));
        log!(message);
        std::thread::sleep(Duration::new(0, 250000));
    }
}

#[cfg(test)]
mod validator {
    use std::{fs::OpenOptions, io::Read};
    use sha2::{Digest, Sha256};
    
    #[test]
    fn validate() {
        let mut file = OpenOptions::new()
            .read(true)
            .open("./log.txt").unwrap();
        let mut content = String::new();
        match file.read_to_string(&mut content) {
            Ok(bytes) => println!("read ({}) bytes", bytes),
            Err(err) => println!("could not read file: {}", err.to_string()),
        }
        let mut line_index = 0;
        println!("contents: {}", content);
        for line in content.lines() {
            let mut hasher = Sha256::new();
            let mut parts = line.split(";");
            let checksum = parts.next().unwrap();
            let timestamp = parts.next().unwrap();
            let prefix = parts.next().unwrap();
            let message = parts.next().unwrap();

            if line_index != 0 {
                let prev_message = content.lines().nth(line_index - 1).unwrap();
                let mut prev_checksum = Vec::<char>::new();
                for new_char in prev_message.chars() {
                    if new_char != ';' {
                        break;
                    }
                    prev_checksum.push(new_char);
                }
                hasher.update(prev_checksum.iter().collect::<String>());
                hasher.update(format!("{};{};{}", timestamp, prefix, message));
                let digest = hasher.finalize();
                let hex_string: String = digest
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect();
                println!("({}) | {} | {}", line_index + 1, checksum, hex_string);
                assert_eq!(checksum, hex_string);
            } else {
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