pub struct Message<'a>(pub MessageLevel, pub &'a str, pub Option<&'a str>);

impl Message<'_> {
    pub fn is_suppressed(&self) -> bool {
        self.0.is_suppressed()
    }

    pub fn prefix(&self) -> &str {
        match self.0 {
            MessageLevel::FATAL => MessageLevel::FATAL_PREFIX,
            MessageLevel::ERROR => MessageLevel::ERROR_PREFIX,
            MessageLevel::WARN => MessageLevel::WARN_PREFIX,
            MessageLevel::INFO => MessageLevel::INFO_PREFIX,
            MessageLevel::DEBUG => MessageLevel::DEBUG_PREFIX,
            MessageLevel::TRACE => MessageLevel::TRACE_PREFIX,
            MessageLevel::CUSTOM(_) => self.2.unwrap_or(MessageLevel::CUSTOM_PREFIX),
        }
    }

    pub fn log(&self) {
        use crate::log;
        log!(self)
    }
}

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MessageLevel {
    FATAL = LogLevel::FATAL,
    ERROR = LogLevel::ERROR,
    WARN = LogLevel::WARN,
    INFO = LogLevel::INFO,
    DEBUG = LogLevel::DEBUG,
    TRACE = LogLevel::TRACE,
    CUSTOM(u16)
}

impl MessageLevel {
    pub(crate) const FATAL_PREFIX: &str = "\x1b[31mFATAL\x1b[0m";
    pub(crate) const ERROR_PREFIX: &str = "\x1b[31mERROR\x1b[0m";
    pub(crate) const WARN_PREFIX: &str = "\x1b[33mWARN\x1b[0m";
    pub(crate) const DEBUG_PREFIX: &str = "\x1b[34mDEBUG\x1b[0m";
    pub(crate) const INFO_PREFIX: &str = "\x1b[32mINFO\x1b[0m";
    pub(crate) const TRACE_PREFIX: &str = "\x1b[90mTRACE\x1b[0m";
    pub(crate) const CUSTOM_PREFIX: &str = "\x1b[90mCUSTOM\x1b[0m";
}

pub struct LogLevel(pub u16);

impl LogLevel {
    const FATAL: u16 = 0;
    const ERROR: u16 = 100;
    const WARN: u16 = 200;
    const INFO: u16 = 300;
    const DEBUG: u16 = 400;
    const TRACE: u16 = 500;

    pub fn fatal() -> u16 {
        Self::FATAL
    }
    
    pub fn error() -> u16 {
        Self::ERROR
    }
    
    pub fn warn() -> u16 {
        Self::WARN
    }
    
    pub fn info() -> u16 {
        Self::INFO
    }
    
    pub fn debug() -> u16 {
        Self::DEBUG
    }
    
    pub fn trace() -> u16 {
        Self::TRACE
    }
}

impl MessageLevel {
    pub fn as_u16(&self) -> u16 {
        match self {
            MessageLevel::FATAL => LogLevel::FATAL,
            MessageLevel::ERROR => LogLevel::ERROR,
            MessageLevel::WARN  => LogLevel::WARN,
            MessageLevel::INFO  => LogLevel::INFO,
            MessageLevel::DEBUG => LogLevel::DEBUG,
            MessageLevel::TRACE => LogLevel::TRACE,
            MessageLevel::CUSTOM(v) => *v,
        }
    }

    pub(crate) fn is_suppressed(&self) -> bool {
        if let Some(log_level) = crate::LOG_LEVEL.get() {
            return *log_level <= self.as_u16()
        } else {return false}
    }
}

impl From<u16> for MessageLevel {
    fn from(value: u16) -> Self {
        match value {
            LogLevel::FATAL => MessageLevel::FATAL,
            LogLevel::ERROR => MessageLevel::ERROR,
            LogLevel::WARN => MessageLevel::WARN,
            LogLevel::INFO => MessageLevel::INFO,
            LogLevel::DEBUG => MessageLevel::DEBUG,
            LogLevel::TRACE => MessageLevel::TRACE,
            v => MessageLevel::CUSTOM(v)
        }
    }
}

impl From<LogLevel> for MessageLevel {
    fn from(value: LogLevel) -> Self {
        Self::from(value.0)
    }
}