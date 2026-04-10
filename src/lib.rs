use std::{env, fs::OpenOptions, sync::{Arc, Mutex, OnceLock}, thread};

use crate::{logger::Logger, util::level::Message};

pub mod logger;
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
    let log_level: u16 = u16::from_str_radix(&env::var("EZENV_LOG_LEVEL").unwrap_or_default(), 10).unwrap_or(500);
    let destination: u8 = u8::from_str_radix(&env::var("EZENV_LOG_DESTINATION").unwrap_or_default(), 2).unwrap_or(LogDestination::Default.bits());
    let output_path = env::var("EZENV_OUTPUT_FILE").unwrap_or("./log.txt".to_string());
    let file = match OpenOptions::new()
        .create(true)
        .append(true)
        .open(output_path.clone()) {
            Ok(file) => Some(file),
            Err(_) => None,
        };
    let logger = Logger{
        destination,
        log_level,
        file,
    };
    let _ = MESSAGE_QUEUE.set(Arc::new(Mutex::new(Vec::new())));
    match LOGGER.set(Arc::new(Mutex::new(logger))) {
        Ok(_) => {
            use crate::info;
            info!("Logger initialized!");
        },
        Err(_) => eprintln!("Logger already initialized!"),
    };
    thread::spawn(move || {
        loop {
            let mut queue = MESSAGE_QUEUE.get().unwrap().lock().unwrap();
            if queue.len() > 0 {
                let logger = LOGGER.get().unwrap().lock().unwrap();
                let mut iter = queue.clone().into_iter();
                while let Some(message) = iter.next() {
                    logger.log(message);
                }
                queue.clear();
            }
        }
    });
}

#[macro_export]
macro_rules! fatal {
    ( $( $arg:tt )* ) => {{
        use crate::util::level::{Message, MessageLevel};
        use crate::MESSAGE_QUEUE;
        use std::thread;
        
        thread::spawn(move || {
            let queue = &MESSAGE_QUEUE.get().unwrap();
            let msg = format!($($arg)*);
            let message = Message(MessageLevel::FATAL, msg, None);
            queue.lock().unwrap().push(message);
        });
    }};
}

#[macro_export]
macro_rules! error {
    ( $( $arg:tt )* ) => {{
        use crate::util::level::{Message, MessageLevel};
        use crate::MESSAGE_QUEUE;
        use std::thread;
        
        thread::spawn(move || {
            let queue = &MESSAGE_QUEUE.get().unwrap();
            let msg = format!($($arg)*);
            let message = Message(MessageLevel::ERROR, msg, None);
            queue.lock().unwrap().push(message);
        });
    }};
}

#[macro_export]
macro_rules! warn {
    ( $( $arg:tt )* ) => {{
        use crate::util::level::{Message, MessageLevel};
        use crate::MESSAGE_QUEUE;
        use std::thread;
        
        thread::spawn(move || {
            let queue = &MESSAGE_QUEUE.get().unwrap();
            let msg = format!($($arg)*);
            let message = Message(MessageLevel::WARN, msg, None);
            queue.lock().unwrap().push(message);
        });
    }};
}

#[macro_export]
macro_rules! info {
    ( $( $arg:tt )* ) => {{
        use crate::util::level::{Message, MessageLevel};
        use crate::MESSAGE_QUEUE;
        use std::thread;
        
        thread::spawn(move || {
            let queue = &MESSAGE_QUEUE.get().unwrap();
            let msg = format!($($arg)*);
            let message = Message(MessageLevel::INFO, msg, None);
            queue.lock().unwrap().push(message);
        });
    }};
}

#[macro_export]
macro_rules! debug {
    ( $( $arg:tt )* ) => {{
        use crate::util::level::{Message, MessageLevel};
        use crate::MESSAGE_QUEUE;
        use std::thread;
        
        thread::spawn(move || {
            let queue = &MESSAGE_QUEUE.get().unwrap();
            let msg = format!($($arg)*);
            let message = Message(MessageLevel::DEBUG, msg, None);
            queue.lock().unwrap().push(message);
        });
    }};
}

#[macro_export]
macro_rules! trace {
    ( $( $arg:tt )* ) => {{
        use crate::util::level::{Message, MessageLevel};
        use crate::MESSAGE_QUEUE;
        use std::thread;
        
        thread::spawn(move || {
            let queue = &MESSAGE_QUEUE.get().unwrap();
            let msg = format!($($arg)*);
            let message = Message(MessageLevel::TRACE, msg, None);
            queue.lock().unwrap().push(message);
        });
    }};
}

#[macro_export]
macro_rules! log {
    ($msg:expr) => {{
        use crate::{fatal, error, warn, info, debug, trace};
        let Message(level, message, prefix) = $msg;
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
                use crate::util::level::{Message, MessageLevel};
                use crate::MESSAGE_QUEUE;
                use std::thread;
                
                thread::spawn(move || {
                    let prefix = prefix.unwrap_or(MessageLevel::CUSTOM_PREFIX);
                    let queue = &MESSAGE_QUEUE.get().unwrap();
                    queue.lock().unwrap().push(Message(level, message, Some(prefix)));
                });
            }
        }
    }};
}