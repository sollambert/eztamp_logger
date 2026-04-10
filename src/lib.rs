use std::{env, fs::{File, OpenOptions}, sync::OnceLock};

pub mod tests;
pub mod util;

pub(crate) static LOG_LEVEL: OnceLock<u16> = OnceLock::new();
pub(crate) static LOG_FILE: OnceLock<File> = OnceLock::new();
pub(crate) static LOG_DESTINATION: OnceLock<u8> = OnceLock::new();


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

    pub(crate) fn is_default() -> bool  {
        if let Some(mask) = LOG_DESTINATION.get() {
            return *mask & LogDestination::Default.bits() != 0;
        }
        false
    }

    pub(crate) fn is_std_err() -> bool  {
        if let Some(mask) = LOG_DESTINATION.get() {
            return *mask & LogDestination::StdErr.bits() != 0;
        }
        false
    }

    pub(crate) fn is_std_out() -> bool  {
        if let Some(mask) = LOG_DESTINATION.get() {
            return *mask & LogDestination::StdOut.bits() != 0;
        }
        false
    }

    pub(crate) fn file_logging() -> bool  {
        if let Some(mask) = LOG_DESTINATION.get() {
            return *mask & LogDestination::File.bits() != 0;
        }
        false
    }
}

pub fn init() {
    let log_level: u16 = u16::from_str_radix(&env::var("EZENV_LOG_LEVEL").unwrap_or_default(), 10).unwrap_or(400);
    match LOG_LEVEL.set(log_level) {
        Ok(_) => println!("Log level set: {}", log_level),
        Err(_) => eprintln!("Could not set log_level!"),
    }
    let log_destinattion: u8 = u8::from_str_radix(&env::var("EZENV_LOG_DESTINATION").unwrap_or_default(), 2).unwrap_or(LogDestination::Default.bits());
    match LOG_DESTINATION.set(log_destinattion) {
        Ok(_) => println!("Log destination set: {}", log_destinattion),
        Err(_) => eprintln!("Could not set destination!"),
    }
    let output_path = env::var("EZENV_OUTPUT_FILE").unwrap_or("./log.txt".to_string());
    if let Ok(log_file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(output_path.clone()) {
        match LOG_FILE.set(log_file) {
            Ok(_) => println!("Log output file ready: {}", &output_path),
            Err(_) => eprintln!("Could not access output file!"),
        }
    }
}

#[macro_export]
macro_rules! fatal {
    ( $( $arg:tt )* ) => {{
        use std::io::Write;
        use crate::{LogDestination, util::{level::MessageLevel, time::utc_timestamp}};
        let timestamp = utc_timestamp();

        let msg = format!($($arg)*);
        const PREFIX: &str = MessageLevel::FATAL_PREFIX;


        if !MessageLevel::FATAL.is_suppressed() {
            if LogDestination::is_default() || LogDestination::is_std_err() {
                eprintln!("[{}][{}]: {}", timestamp, PREFIX, msg);
            }
            if LogDestination::is_std_out() {
                println!("[{}][{}]: {}", timestamp, PREFIX, msg);
            }

            if LogDestination::file_logging() {
                if let Some(file) = crate::LOG_FILE.get() {
                    let mut file = file;
                    let _ = writeln!(file, "[{}][FATAL]: {}",timestamp, msg);
                }
            }
        }
    }};
}
#[macro_export]
macro_rules! error {
    ( $( $arg:tt )* ) => {{
        use std::io::Write;
        use crate::{LogDestination, util::{level::MessageLevel, time::utc_timestamp}};

        let msg = format!($($arg)*);
        const PREFIX: &str = MessageLevel::ERROR_PREFIX;
        let timestamp = utc_timestamp();

        if !MessageLevel::ERROR.is_suppressed() {
            if LogDestination::is_default() || LogDestination::is_std_err() {
                eprintln!("[{}][{}]: {}", timestamp, PREFIX, msg);
            }
            if LogDestination::is_std_out() {
                println!("[{}][{}]: {}", timestamp, PREFIX, msg);
            }
            if LogDestination::file_logging() {
                if let Some(file) = crate::LOG_FILE.get() {
                    let mut file = file;
                    let _ = writeln!(file, "[{}][ERROR]: {}",timestamp, msg);
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! warn {
    ( $( $arg:tt )* ) => {{
        use std::io::Write;
        use crate::{LogDestination, util::{level::MessageLevel, time::utc_timestamp}};

        let msg = format!($($arg)*);
        const PREFIX: &str = MessageLevel::WARN_PREFIX;
        let timestamp = utc_timestamp();

        if !MessageLevel::WARN.is_suppressed() {
            if LogDestination::is_std_err() {
                eprintln!("[{}][{}]: {}", timestamp, PREFIX, msg);
            }
            if LogDestination::is_default() || LogDestination::is_std_out() {
                println!("[{}][{}]: {}", timestamp, PREFIX, msg);
            }

            if LogDestination::file_logging() {
                if let Some(file) = crate::LOG_FILE.get() {
                    let mut file = file;
                    let _ = writeln!(file, "[{}][WARN]: {}",timestamp, msg);
                }
            }
        }
    }};
}
#[macro_export]
macro_rules! info {
    ( $( $arg:tt )* ) => {{
        use std::io::Write;
        use crate::{LogDestination, util::{level::MessageLevel, time::utc_timestamp}};

        let msg = format!($($arg)*);
        const PREFIX: &str = MessageLevel::INFO_PREFIX;
        let timestamp = utc_timestamp();

        if !MessageLevel::INFO.is_suppressed() {
            if LogDestination::is_std_err() {
                eprintln!("[{}][{}]: {}", timestamp, PREFIX, msg);
            }
            if LogDestination::is_default() || LogDestination::is_std_out() {
                println!("[{}][{}]: {}", timestamp, PREFIX, msg);
            }

            if LogDestination::file_logging() {
                if let Some(file) = crate::LOG_FILE.get() {
                    let mut file = file;
                    let _ = writeln!(file, "[{}][INFO]: {}",timestamp, msg);
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! debug {
    ( $( $arg:tt )* ) => {{
        use std::io::Write;
        use crate::{LogDestination, util::{level::MessageLevel, time::utc_timestamp}};

        let msg = format!($($arg)*);
        const PREFIX: &str = MessageLevel::DEBUG_PREFIX;
        let timestamp = utc_timestamp();

        if !MessageLevel::DEBUG.is_suppressed() {
            if LogDestination::is_std_err() {
                eprintln!("[{}][{}]: {}", timestamp, PREFIX, msg);
            }
            if LogDestination::is_default() || LogDestination::is_std_out() {
                println!("[{}][{}]: {}", timestamp, PREFIX, msg);
            }

            if LogDestination::file_logging() {
                if let Some(file) = crate::LOG_FILE.get() {
                    let mut file = file;
                    let _ = writeln!(file, "[{}][DEBUG]: {}",timestamp, msg);
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! trace {
    ( $( $arg:tt )* ) => {{
        use std::io::Write;
        use crate::{LogDestination, util::{level::MessageLevel, time::utc_timestamp}};

        let msg = format!($($arg)*);
        const PREFIX: &str = MessageLevel::TRACE_PREFIX;
        let timestamp = utc_timestamp();

        if !MessageLevel::TRACE.is_suppressed() {
            if LogDestination::is_std_err() {
                eprintln!("[{}][{}]: {}", timestamp, PREFIX, msg);
            }
            if LogDestination::is_default() || LogDestination::is_std_out() {
                println!("[{}][{}]: {}", timestamp, PREFIX, msg);
            }
            if LogDestination::file_logging() {
                if let Some(file) = crate::LOG_FILE.get() {
                    let mut file = file;
                    let _ = writeln!(file, "[{}][TRACE]: {}",timestamp, msg);
                }
            }
        }
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
            MessageLevel::CUSTOM(v) => {
                use std::io::Write;
                use crate::{LogDestination, util::{level::MessageLevel, time::utc_timestamp}};
                
                let timestamp = utc_timestamp();
                let prefix = prefix.unwrap_or(MessageLevel::CUSTOM_PREFIX);

                if !level.is_suppressed() {
                    if LogDestination::is_std_err() {
                        eprintln!("[{}][{}({})]: {}", timestamp, prefix, v, message);
                    }
                    if LogDestination::is_default() || LogDestination::is_std_out() {
                        println!("[{}][{}({})]: {}", timestamp, prefix, v, message);
                    }

                    if LogDestination::file_logging() {
                        if let Some(file) = crate::LOG_FILE.get() {
                            let mut file = file;
                            let _ = writeln!(file, "[{}][CUSTOM({})]: {}",timestamp, v, message);
                        }
                    }
                }
            }
        }
    }};
}