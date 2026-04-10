#[cfg(test)]
mod tests {
    use crate::{error, fatal, info, log, trace, util::level::{Message, MessageLevel}, warn};
    #[test]
    fn fatal() {
        crate::init();
        fatal!("This is a fatal error!")
    } 
    #[test]
    fn error() {
        crate::init();
        error!("This is an error!")
    } 
    #[test]
    fn warn() {
        crate::init();
        warn!("This is a warn!")
    } 
    #[test]
    fn info() {
        crate::init();
        info!("This is a info message!")
    } 
    #[test]
    fn trace() {
        crate::init();
        trace!("This is a trace message!")
    } 
    #[test]
    fn custom() {
        crate::init();
        let message = Message(MessageLevel::CUSTOM(320), "This is a custom message!", Some("TEST"));
        log!(message);
    }
}