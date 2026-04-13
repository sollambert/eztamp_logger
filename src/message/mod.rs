use crate::{message::level::MessageLevel, util::time::TimeStamp};

pub mod level;

#[derive(Clone)]
pub struct Message(TimeStamp, pub(crate) MessageLevel, pub(crate) String, pub(crate) MessagePrefix);

impl Message {
    pub fn new(level: MessageLevel, text: String, prefix: MessagePrefix) -> Self {
        Self(TimeStamp::now(), level, text, prefix)
    }

    pub fn level(&self) -> MessageLevel {
        self.1
    }

    pub fn text(&self) -> String {
        self.2.clone()
    }

    pub fn timestamp(&self) -> TimeStamp {
        self.0
    }

    pub fn prefix(&self) -> MessagePrefix {
        self.3.clone()
    }

    pub fn is_suppressed(&self, log_level: u16) -> bool {
        self.1.is_suppressed(log_level)
    }

    pub fn to_display_string(&self) -> String {
        match self.level() {
            MessageLevel::CUSTOM(v) => format!("[{}][{}({})]: {}", &self.timestamp().as_utc(), &self.prefix().to_string(), v, self.text()),
            _ => format!("[{}][{}]: {}", &self.timestamp().as_utc(), &self.prefix().to_string(), self.text())
        }
    }

    pub fn to_string(&self) -> String {
        format!("{};{};{}", &self.timestamp().as_utc(), &self.prefix().text, self.text())
    }
}

#[derive(Clone)]
pub enum PrefixColor {
    Red,
    Yellow,
    Blue,
    Green,
    Gray,
    None
}

impl PrefixColor {
    pub fn to_parts(&self) -> (&str, &str) {
        match self {
            PrefixColor::Red => ("\x1b[31m", "\x1b[0m"),
            PrefixColor::Green => ("\x1b[32m", "\x1b[0m"),
            PrefixColor::Yellow => ("\x1b[33m", "\x1b[0m"),
            PrefixColor::Blue => ("\x1b[34m", "\x1b[0m"),
            PrefixColor::Gray => ("\x1b[90m", "\x1b[0m"),
            PrefixColor::None => ("", "")
        }
    }
}

#[derive(Clone)]
pub struct MessagePrefix {
    color: PrefixColor,
    text: &'static str
}

impl MessagePrefix {
    pub const FATAL_PREFIX: Self = Self {color: PrefixColor::Red, text: "FATAL"};
    pub const ERROR_PREFIX: Self = Self {color: PrefixColor::Red, text: "ERROR"};
    pub const WARN_PREFIX: Self = Self {color: PrefixColor::Yellow, text: "WARN"};
    pub const INFO_PREFIX: Self = Self {color: PrefixColor::Green, text: "INFO"};
    pub const DEBUG_PREFIX: Self = Self {color: PrefixColor::Blue, text: "DEBUG"};
    pub const TRACE_PREFIX: Self = Self {color: PrefixColor::Gray, text: "TRACE"};

    pub fn to_string(&self) -> String {
        let (front, back) = self.color.to_parts();
        format!("{}{}{}", front, self.text, back)
    }

    pub fn custom_prefix(text: &'static str) -> Self {
        Self {
            color: PrefixColor::Gray,
            text
        }
    }
}