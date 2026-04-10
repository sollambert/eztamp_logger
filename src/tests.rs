#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{debug, error, fatal, info, trace, util::level::{Message, MessageLevel}, warn};

    #[test]
    fn all() {
        crate::init();
        fatal!("This is a fatal error!");
        error!("This is an error!");
        warn!("This is a warn!");
        info!("This is an info message!");
        debug!("This is a debug message!");
        trace!("This is a trace message!");
        std::thread::sleep(Duration::new(0, 50000));
    }

    #[test]
    fn custom() {
        use crate::LOGGER;
        crate::init();
        let binding = &LOGGER;
        let logger = binding.get().unwrap();
        let message = Message(MessageLevel::CUSTOM(320), "This is a custom message!".to_string(), Some("TEST"));
        logger.lock().unwrap().log(message);
    }
}