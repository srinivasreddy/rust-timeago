## README
This is an yet another implementation of `timeago` time display.This library is mostly
inspired from the strings you see on stackoverflow.com

## Design
I haven't considered the leap second into design and i have taken generally 30 days as month.
So you can assume that i have sacrificed accuracy of months at the altar of usability. An year is 
considered as 365.25 days long.

## TODO.
1. Haven't implemented tests for strings such as `"Nov 1970 at 17:20:08"`. I need to use Faketime library
to mock the current system time.

## Usage

```rust
use time_ago::{Config, TimeAgo, TimeType};
fn usage() {
    let config = Config { 
        is_years: false,
        is_weeks: false,
        is_months: false,
    };
    let a = TimeAgo::with_config(config, TimeType::SystemTime(SystemTime::now()));
    assert_eq!(a.convert(), "just now");
}
```

You can also see the [tests](src/tests.rs) file for more usage.
### License
MIT or Apache

### copyright
Srinivas Reddy Thatiparthy


