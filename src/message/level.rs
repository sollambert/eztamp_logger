

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

pub struct LogLevel(pub u16);

impl LogLevel {
    const FATAL: u16 = 1;
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

    pub(crate) fn is_suppressed(&self, log_level: u16) -> bool {
        return log_level < self.as_u16();
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