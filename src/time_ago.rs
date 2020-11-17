use std::time::{Duration, SystemTime};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimeAgo {
    config: Config,
    time_type: TimeType,
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TimeType {
    SystemTime(SystemTime),
    Duration(Duration),
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Config {
    pub is_weeks: bool,
    pub is_months: bool,
    pub is_years: bool,
}

// Default implementation of Config
// is_weeks: false -> "23 day(s) ago" is displayed instead of "1 week(s) ago".
// is_months: false -> "Nov 20 at 11:30"  is displayed instead of "1 month(s) ago".
// is_years: false -> "Nov 10 at 21:23" is displayed instead of "10 years ago"
impl Default for Config {
    fn default() -> Self {
        Config {
            is_weeks: false,
            is_months: false,
            is_years: false,
        }
    }
}

impl TimeAgo {
    pub fn with_config(config: Config, time_type: TimeType) -> TimeAgo {
        TimeAgo { config, time_type }
    }
    pub fn from_duration(config: Config, duration: Duration) -> TimeAgo {
        TimeAgo {
            config,
            time_type: TimeType::Duration(duration),
        }
    }

    pub fn now(config: Config) -> TimeAgo {
        Self::from_system_time(config, SystemTime::now())
    }
    pub fn from_system_time(config: Config, system_time: SystemTime) -> TimeAgo {
        TimeAgo {
            config,
            time_type: TimeType::SystemTime(system_time),
        }
    }

    pub fn convert(&self) -> String {
        let seconds = match &self.time_type {
            TimeType::SystemTime(value) => {
                value.duration_since(SystemTime::now()).unwrap().as_secs()
            }
            TimeType::Duration(value) => value.as_secs(),
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
            (7200..=86_399) => format!("{} hours ago", seconds / 60 / 60),
            //1 day to 1 day 23 hours 59 minutes 59 seconds,
            (86_400..=172_799) => "yesterday".to_string(),
            //2 days to 6 days 23 hours 59 minutes 59 seconds
            (172_800..=604_799) => format!("{} days ago", seconds / 60 / 60 / 24),
            //1 week to 1 week 6 days 23 hours 59 minutes 59 seconds
            (604_800..=1_209_599) => {
                if &self.config.is_weeks {
                    "1 week ago".to_string()
                } else {
                    format!("{} days ago", seconds / 60 / 60 / 24)
                }
            }
            //2 weeks to 29 days 23 hours 59 minutes 59 seconds
            // (1209600..)
            //1 month to 1 month 29 days 23 hours 59 minutes 59 seconds
            //2 months to 11 months 29 days 23 hours 59 minutes 59 seconds
            // 1 year to 11 months 29 days 23 hours 59 minutes 59 seconds
            // 2 years to 99 years ago.
            _ => "Hola!!!".to_string(),
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
