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