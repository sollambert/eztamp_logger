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
    use std::{env, fs::OpenOptions};

    use crate::logger::verify_log;
    
    #[test]
    fn validate() {
        let _ = dotenv::dotenv();
        let secret = env::var("RUST_LOG_SECRET").unwrap_or_default();
        println!("Validating with initial secret: {}", secret);
        let output_path = env::var("RUST_LOG_OUTPUT_FILE").unwrap_or("./log.txt".to_string());
        let file = OpenOptions::new()
            .read(true)
            .open(output_path).unwrap();
        match verify_log(file, secret) {
            Ok(_) => assert!(true),
            Err(err) => panic!("{}", err),
        }
        println!("Log validated, no checksum mismatch detected")
    }
}