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
            "1 day ago"
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
                TimeType::Duration(Duration::from_secs(60 * 60 * 24 * 30 * 12)) // duration calculates 365.25* 60 * 60 * 24 for an year
            )
            .convert(),
            "11 months ago"
        );

        assert_eq!(
            TimeAgo::with_config(
                custom,
                TimeType::Duration(Duration::from_secs(60 * 60 * 24 * 366)) // duration calculates 365.25* 60 * 60 * 24 for an year
            )
            .convert(),
            "1 year ago"
        );

        assert_eq!(
            TimeAgo::with_config(
                custom,
                TimeType::Duration(Duration::from_secs(
                    (60.0 * 60.0 * 24.0 * 365.25 * 2.0) as u64 - 1
                )) // duration calculates 365.25* 60 * 60 * 24 for an year
            )
            .convert(),
            "1 year ago"
        );

        assert_eq!(
            TimeAgo::with_config(
                custom,
                TimeType::Duration(Duration::from_secs(
                    (60.0 * 60.0 * 24.0 * 365.25 * 2.0) as u64
                )) // duration calculates 365.25* 60 * 60 * 24 for an year
            )
            .convert(),
            "2 years ago"
        );

        assert_eq!(
            TimeAgo::with_config(
                custom,
                TimeType::Duration(Duration::from_secs(
                    (60.0 * 60.0 * 24.0 * 365.25 * 51.0) as u64
                )) // duration calculates 365.25* 60 * 60 * 24 for an year
            )
            .convert(),
            "51 years ago"
        );
        assert_eq!(
            TimeAgo::with_config(
                custom,
                TimeType::Duration(Duration::from_secs(
                    (60.0 * 60.0 * 24.0 * 365.25 * 100.0) as u64
                )) // duration calculates 365.25* 60 * 60 * 24 for an year
            )
            .convert(),
            "invalid string"
        );
        // assert_eq!(
        //     TimeAgo::with_config(
        //         Config{
        //             is_months: false,
        //             is_weeks: false,
        //             is_years: false
        //         },
        //         TimeType::Duration(Duration::from_secs(
        //             (60.0 * 60.0 * 24.0 * 365.25 * 50.0) as u64
        //         )) // duration calculates 365.25* 60 * 60 * 24 for an year
        //     )
        //         .convert(),
        //     "Nov 1970 at 17:07:20"
        // );
    }
}
