# Active Support -- Utility extensions for Rust

Active Support is a collection of utility extensions for Rust inspired by activesupport of Ruby on Rails

## License

Active Support is released under the MIT license:

* https://opensource.org/licenses/MIT


## Support

* https://docs.rs/activesupport/

## Usage example

```rust
use activesupport::time::{Duration, TimeDuration, TimeRange, Utc};

fn main() {
    assert_eq!(60.seconds(), Duration::seconds((60).into()));
    assert_eq!(1.second(), Duration::seconds((1).into()));
    assert_eq!(60.minutes(), Duration::minutes((60).into()));
    assert_eq!(1.minute(), Duration::minutes((1).into()));
    assert_eq!(60.hours(), Duration::hours((60).into()));
    assert_eq!(1.hour(), Duration::hours((1).into()));
    assert_eq!(60.days(), Duration::days((60).into()));
    assert_eq!(1.day(), Duration::days((1).into()));
    assert_eq!(60.weeks(), Duration::weeks((60).into()));
    assert_eq!(1.week(), Duration::weeks((1).into()));
    assert_eq!(60.fortnights(), Duration::weeks((120).into()));
    assert_eq!(1.fortnight(), Duration::weeks((2).into()));

    assert!(1.day().from_now() <= Utc::now().checked_add_signed(1.day().to_owned()));
    assert!(2.weeks().from_now() <= Utc::now().checked_add_signed(2.weeks().to_owned()));
    assert!(
        (4.days() + 5.weeks()).from_now()
            <= Utc::now().checked_add_signed((4.days() + 5.weeks()).to_owned())
    );

    assert!(1.day().ago() <= Utc::now().checked_sub_signed(1.day().to_owned()));
    assert!(2.weeks().ago() <= Utc::now().checked_sub_signed(2.weeks().to_owned()));
    assert!(
        (4.days() + 5.weeks()).ago()
            <= Utc::now().checked_sub_signed((4.days() + 5.weeks()).to_owned())
    );
}
```