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
            // 0 to 1 second is just now
            (0..=1) => "just now".to_string(),
            // 2 to 59 seconds ago
            (2..=59) => format!("{} seconds ago", seconds),
            // 60 seconds to 119 seconds
            (60..=119) => "1 minute ago".to_string(),
            // 2 minutes to 59 minutes 59 seconds
            (120..=3599) => format!("{} minutes ago", seconds / 60),
            //1 hour to 2 hours
            (3600..=7199) => "1 hour ago".to_string(),
            //2 hours to 23 hours 59 minutes 59 seconds
            (7200..=86399) => format!("{} hours ago", seconds / 60 / 60),
            //1 day to 1 day 23 hours 59 minutes 59 seconds,
            (86400..=172799) => "1 day ago".to_string(),
            //2 days to 6 days 23 hours 59 minutes 59 seconds
            (172800..=604799) => format!("{} days ago", seconds / 60 / 60 / 24),
            //1 week to 1 week 6 days 23 hours 59 minutes 59 seconds
            (604800..=1209599) => "1 week ago".to_string(),
            //2 weeks to 29 days 23 hours 59 minutes 59 seconds
            // (1209600..)
            //1 month to 1 month 29 days 23 hours 59 minutes 59 seconds
            //2 months to 11 months 29 days 23 hours 59 minutes 59 seconds
            // 1 year to 11 months 29 days 23 hours 59 minutes 59 seconds
            // 2 years to 99 years ago.
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
