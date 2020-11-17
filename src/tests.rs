#[cfg(test)]
mod tests {
    use crate::time_ago::{Config, TimeAgo, TimeType};
    use std::time::{Duration, Instant, SystemTime};

    #[test]
    fn test_timeago_with_config_system_time() {
        let a = TimeAgo::with_config(
            Config {
                is_years: false,
                is_weeks: false,
                is_months: false,
            },
            TimeType::Instant(Instant::now()),
        );
        assert_eq!(a.convert(), "just now");
        let b = TimeAgo::with_config(
            Config {
                is_years: false,
                is_weeks: false,
                is_months: false,
            },
            TimeType::Instant(Instant::now()),
        );
        assert_eq!(b.convert(), "just now");
    }

    #[test]
    fn test_timeago_with_config_duration() {
        let config = Config {
            is_years: false,
            is_weeks: false,
            is_months: false,
        };
        let b = TimeAgo::with_config(config, TimeType::Duration(Duration::from_secs(1)));
        assert_eq!(b.convert(), "just now");

        let c = TimeAgo::with_config(config, TimeType::Duration(Duration::from_secs(1)));
        assert_eq!(c.convert(), "just now");
    }
}
