use std::fmt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TimeAgo {
    SystemTime(SystemTime),
    Duration(Duration),
}

impl TimeAgo {
    pub fn duration(duration: Duration) -> TimeAgo {
        TimeAgo::Duration(duration)
    }

    pub fn now() -> TimeAgo {
        TimeAgo::SystemTime(SystemTime::now())
    }
    pub fn from_system_time(system_time: SystemTime) -> TimeAgo {
        TimeAgo::SystemTime(system_time)
    }

    pub fn convert(&self) -> String {
        let seconds = match &self {
            TimeAgo::SystemTime(value) => {
                value.duration_since(SystemTime::now()).unwrap().as_secs()
            }
            TimeAgo::Duration(value) => value.as_secs(),
        };
        match seconds {
            (0..=1) => "just now".to_string(),
            (2..=59) => format!("{} seconds ago", seconds),
            (60..=119) => "1 minute ago".to_string(),
            (120..=3600) => format!("{} minutes ago", seconds),
            _ => seconds.to_string(),
        }
    }
    //
    // pub fn convert_with_locale(&self, String: _locale) -> String {
    //     match &self {
    //         TimeAgo::SystemTime(_value) => "Helloworld!!!".to_string(),
    //         TimeAgo::Duration(_value) => "Helloworld!!!".to_string(),
    //     }
    // }
}
