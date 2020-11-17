#[cfg(test)]
mod tests {
    use crate::time_ago::{Config, TimeAgo, TimeType};
    use std::time::{Duration, Instant, SystemTime};

    #[test]
    fn test_time_ago_with_config_system_time() {
        let a = TimeAgo::with_config(
            Config {
                is_years: false,
                is_weeks: false,
                is_months: false,
            },
            TimeType::SystemTime(SystemTime::now()),
        );
        assert_eq!(a.convert(), "just now");
    }

    #[test]
    fn test_time_ago_with_config_duration() {
        let config = Config {
            is_years: false,
            is_weeks: false,
            is_months: false,
        };
        let b = TimeAgo::with_config(config, TimeType::Duration(Duration::from_secs(1)));
        assert_eq!(b.convert(), "just now");
    }

    #[test]
    fn test_time_ago_with_config_instant() {
        let config = Config {
            is_years: false,
            is_weeks: false,
            is_months: false,
        };
        let b = TimeAgo::with_config(config, TimeType::Instant(Instant::now()));
        assert_eq!(b.convert(), "just now");
    }
}
