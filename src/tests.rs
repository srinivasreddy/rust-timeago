#[cfg(test)]
mod tests {
    use crate::time_ago::{Config, TimeAgo, TimeType};
    use std::time::{Duration, Instant, SystemTime};

    #[test]
    fn test_time_ago_with_config() {
        let config = Config {
            is_years: false,
            is_weeks: false,
            is_months: false,
        };

        let custom = Config {
            is_years: true,
            is_weeks: true,
            is_months: true,
        };
        let a = TimeAgo::with_config(config, TimeType::SystemTime(SystemTime::now()));
        assert_eq!(a.convert(), "just now");
        let b = TimeAgo::with_config(config, TimeType::Duration(Duration::from_secs(1)));
        assert_eq!(b.convert(), "just now");
        let c = TimeAgo::with_config(config, TimeType::Instant(Instant::now()));
        assert_eq!(c.convert(), "just now");
        assert_eq!(
            TimeAgo::with_config(custom, TimeType::Duration(Duration::from_secs(5))).convert(),
            "5 seconds ago"
        );
        assert_eq!(
            TimeAgo::with_config(custom, TimeType::Duration(Duration::from_secs(60))).convert(),
            "1 minute ago"
        );
        assert_eq!(
            TimeAgo::with_config(custom, TimeType::Duration(Duration::from_secs(120))).convert(),
            "2 minutes ago"
        );
        assert_eq!(
            TimeAgo::with_config(custom, TimeType::Duration(Duration::from_secs(60 * 60)))
                .convert(),
            "1 hour ago"
        );
        assert_eq!(
            TimeAgo::with_config(custom, TimeType::Duration(Duration::from_secs(60 * 60 * 2)))
                .convert(),
            "2 hours ago"
        );
        assert_eq!(
            TimeAgo::with_config(
                custom,
                TimeType::Duration(Duration::from_secs(60 * 60 * 23))
            )
            .convert(),
            "23 hours ago"
        );
        assert_eq!(
            TimeAgo::with_config(
                custom,
                TimeType::Duration(Duration::from_secs(60 * 60 * 24))
            )
            .convert(),
            "yesterday"
        );
        assert_eq!(
            TimeAgo::with_config(
                custom,
                TimeType::Duration(Duration::from_secs(60 * 60 * 24 * 7))
            )
            .convert(),
            "1 week ago"
        );
        assert_eq!(
            TimeAgo::with_config(
                custom,
                TimeType::Duration(Duration::from_secs(60 * 60 * 24 * 7 * 3))
            )
            .convert(),
            "3 weeks ago"
        );
        assert_eq!(
            TimeAgo::with_config(
                custom,
                TimeType::Duration(Duration::from_secs((60 * 60 * 24 * 7 * 4) - 1))
            )
            .convert(),
            "3 weeks ago"
        );
        assert_eq!(
            TimeAgo::with_config(
                custom,
                TimeType::Duration(Duration::from_secs(60 * 60 * 24 * 30))
            )
            .convert(),
            "1 month ago"
        );
        assert_eq!(
            TimeAgo::with_config(
                custom,
                TimeType::Duration(Duration::from_secs(60 * 60 * 24 * 30 * 2))
            )
            .convert(),
            "2 months ago"
        );
        assert_eq!(
            TimeAgo::with_config(
                custom,
                TimeType::Duration(Duration::from_secs(60 * 60 * 24 * 30 * 12))
            )
            .convert(),
            "11 months ago"
        );
    }
}
