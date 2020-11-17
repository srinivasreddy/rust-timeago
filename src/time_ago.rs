use std::time::{Duration, Instant, SystemTime};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimeAgo {
    config: Config,
    time_type: TimeType,
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TimeType {
    Instant(Instant),
    Duration(Duration),
    SystemTime(SystemTime),
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
    pub fn from_system_time(config: Config, system_time: SystemTime) -> TimeAgo {
        TimeAgo {
            config,
            time_type: TimeType::SystemTime(system_time),
        }
    }
    pub fn now(config: Config) -> TimeAgo {
        Self::from_instant(config, Instant::now())
    }
    pub fn from_instant(config: Config, instant: Instant) -> TimeAgo {
        TimeAgo {
            config,
            time_type: TimeType::Instant(instant),
        }
    }

    pub fn convert(&self) -> String {
        let seconds = match &self.time_type {
            TimeType::SystemTime(value) => SystemTime::now()
                .duration_since(*value)
                .unwrap_or(Duration::from_secs(0))
                .as_secs(),
            TimeType::Instant(value) => Instant::now().duration_since(*value).as_secs(),
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
            (172_800..=604_799) => format!("{} days ago", seconds / (60 * 60 * 24)),
            //1 week to 1 week 6 days 23 hours 59 minutes 59 seconds
            (604_800..=1_209_599) => {
                if self.config.is_weeks {
                    "1 week ago".to_string()
                } else {
                    // format like "Nov 20 at 10:23"
                    "".to_string()
                }
            }
            //2 weeks to 29 days 23 hours 59 minutes 59 seconds
            (1_209_600..=2_419_199) => {
                if self.config.is_weeks {
                    format!("{} week ago", seconds / (60 * 60 * 24 * 7))
                } else {
                    // format like "Nov 20 at 10:23"
                    "".to_string()
                }
            }
            (2_419_200..4838399) => {
                if self.config.is_months {
                    "1 month ago".to_string()
                } else {
                    match &self.time_type {
                        TimeType::SystemTime(value) => "".to_string(),
                        _ => "".to_string(),
                    }
                }
            } //1 month to 1 month 29 days 23 hours 59 minutes 59 seconds
              //2 months to 11 months 29 days 23 hours 59 minutes 59 seconds
              // 1 year to 11 months 29 days 23 hours 59 minutes 59 seconds
              // 2 years to 99 years ago.
        }
    }
}
