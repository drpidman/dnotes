use std::fmt::Display;

use chrono::prelude::*;

pub enum LogType {
    Info,
    Warn,
    Error
}

pub fn log<T>(log_type: LogType, arg: T)
    where T: Display
{
    match log_type {
        LogType::Info => println!("[INFO][{}]-{}", Utc::now().time(), arg),
        LogType::Warn => println!("[WARN][{}]-{}",  Utc::now().time(), arg),
        LogType::Error => println!("[ERROR][{}]-{}",  Utc::now().time(), arg)
    };
}