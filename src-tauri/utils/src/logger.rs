use colorize;

pub enum LogType {
    Error,
    Warn,
    Info,
    Debug
}

pub struct Log {}

pub trait LogActions {
    fn log(log_type: LogType, msg: &str[]);
}

impl LogActions for Log {
    fn log(log_type: LogType, msg: &str[]) {
           
    }
}
