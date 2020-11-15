#[cfg(test)]
mod tests {
    use crate::time_ago::TimeAgo;
    use std::time::{Duration, SystemTime};

    #[test]
    fn test_time_ago_convert() {
        assert_eq!(TimeAgo::now().convert(), "just now");
        assert_eq!(
            TimeAgo::duration(Duration::from_secs(172900)).convert(),
            "2 days ago"
        )
    }
}
