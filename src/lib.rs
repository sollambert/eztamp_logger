use std::{env, fs::OpenOptions, sync::{Arc, Mutex, OnceLock}, thread};

use crate::{logger::Logger, message::{Message, level::LogLevel}};

pub mod logger;
pub mod message;
pub mod tests;
pub mod util;

pub(crate) static LOGGER: OnceLock<Arc<Mutex<Logger>>> = OnceLock::new();
pub(crate) static MESSAGE_QUEUE: OnceLock<Arc<Mutex<Vec<Message>>>> = OnceLock::new();

#[repr(u8)]
#[derive(Clone, Copy)]
enum LogDestination {
    Default = 0b1001,
    StdErr = 0b0010,
    StdOut = 0b0100,
    File = 0b1000,
}

impl LogDestination {
    fn bits(self) -> u8 {
        self as u8
    }
}

pub fn init() {
    let rust_log = env::var("RUST_LOG").unwrap_or_default();
    let log_level = match rust_log.to_ascii_uppercase().as_str() {
        "TRACE" => LogLevel::trace(),
        "DEBUG" => LogLevel::debug(),
        "INFO" => LogLevel::info(),
        "WARN" => LogLevel::warn(),
        "ERROR" => LogLevel::error(),
        "FATAL" => LogLevel::fatal(),
        "NONE" => 0,
        "" => LogLevel::info(),
        _ =>  u16::from_str_radix(&env::var("RUST_LOG").unwrap_or_default(), 10).unwrap_or(LogLevel::info())
    };
    let secret = env::var("RUST_LOG_SECRET").unwrap_or_default();
    let destination: u8 = u8::from_str_radix(&env::var("RUST_LOG_DESTINATION").unwrap_or_default(), 2).unwrap_or(LogDestination::Default.bits());
    let output_path = env::var("RUST_LOG_OUTPUT_FILE").unwrap_or("./log.txt".to_string());
    let file = match OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(output_path.clone()) {
            Ok(file) => Some(file),
            Err(_) => None
        };
    let logger = Logger{
        destination,
        log_level,
        file,
        secret
    };
    let _ = MESSAGE_QUEUE.set(Arc::new(Mutex::new(Vec::new())));
    match LOGGER.set(Arc::new(Mutex::new(logger))) {
        Ok(_) => {
            use crate::info;
            info!("Logger initialized!");
        },
        Err(_) => eprintln!("Logger already initialized!"),
    };
    thread::spawn(move || loop {
        let mut queue = MESSAGE_QUEUE.get().unwrap().lock().unwrap();
        let mut logger = LOGGER.get().unwrap().lock().unwrap();
        let iter = queue.clone().into_iter();
        iter.for_each(|message| {
            logger.log(message);
        });
        queue.clear();
    });
}

fn queue_message(message: Message) {
    let queue = &MESSAGE_QUEUE.get().unwrap();
    queue.lock().unwrap().push(message);
}

#[macro_export]
macro_rules! fatal {
    ( $( $arg:tt )* ) => {{
        use crate::message::{Message, MessagePrefix, level::MessageLevel};
        use crate::queue_message;
        let msg = format!($($arg)*);
        queue_message(Message::new(MessageLevel::FATAL, msg, MessagePrefix::FATAL_PREFIX));
    }};
}

#[macro_export]
macro_rules! error {
    ( $( $arg:tt )* ) => {{
        use crate::message::{Message, MessagePrefix, level::MessageLevel};
        use crate::queue_message;
        let msg = format!($($arg)*);
        queue_message(Message::new(MessageLevel::ERROR, msg, MessagePrefix::ERROR_PREFIX));
    }};
}

#[macro_export]
macro_rules! warn {
    ( $( $arg:tt )* ) => {{
        use crate::message::{Message, MessagePrefix, level::MessageLevel};
        use crate::queue_message;
        let msg = format!($($arg)*);
        queue_message(Message::new(MessageLevel::WARN, msg, MessagePrefix::WARN_PREFIX));
    }};
}

#[macro_export]
macro_rules! info {
    ( $( $arg:tt )* ) => {{
        use crate::message::{Message, MessagePrefix, level::MessageLevel};
        use crate::queue_message;
        let msg = format!($($arg)*);
        queue_message(Message::new(MessageLevel::INFO, msg, MessagePrefix::INFO_PREFIX));
    }};
}

#[macro_export]
macro_rules! debug {
    ( $( $arg:tt )* ) => {{
        use crate::message::{Message, MessagePrefix, level::MessageLevel};
        use crate::queue_message;
        let msg = format!($($arg)*);
        queue_message(Message::new(MessageLevel::DEBUG, msg, MessagePrefix::DEBUG_PREFIX));
    }};
}

#[macro_export]
macro_rules! trace {
    ( $( $arg:tt )* ) => {{
        use crate::message::{Message, MessagePrefix, level::MessageLevel};
        use crate::queue_message;
        let msg = format!($($arg)*);
        queue_message(Message::new(MessageLevel::TRACE, msg, MessagePrefix::TRACE_PREFIX));
    }};
}

#[macro_export]
macro_rules! log {
    ($msg:expr) => {{
        use crate::{fatal, error, warn, info, debug, trace};
        let level = $msg.level();
        let message = $msg.text();
        let prefix = $msg.prefix();
        match level {
            MessageLevel::FATAL => {
                fatal!("{}", message);
            }
            MessageLevel::ERROR => {
                error!("{}", message);
            }
            MessageLevel::WARN => {
                warn!("{}", message);
            }
            MessageLevel::INFO => {
                info!("{}", message);
            }
            MessageLevel::DEBUG => {
                debug!("{}", message);
            }
            MessageLevel::TRACE => {
                trace!("{}", message);
            }
            MessageLevel::CUSTOM(_) => {
                use crate::message::{Message};
                use crate::queue_message;
                queue_message(Message::new(level, message, prefix));
            }
        }
    }};
}