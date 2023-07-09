pub use chrono::{DateTime, Duration, Utc};

pub trait TimeDuration {
    fn in_milliseconds(&self) -> Duration;

    fn seconds(&self) -> Duration;

    #[inline]
    fn second(&self) -> Duration {
        self.seconds()
    }

    fn minutes(&self) -> Duration;
    #[inline]
    fn minute(&self) -> Duration {
        self.minutes()
    }

    fn hours(&self) -> Duration;
    #[inline]
    fn hour(&self) -> Duration {
        self.hours()
    }

    fn days(&self) -> Duration;
    #[inline]
    fn day(&self) -> Duration {
        self.days()
    }

    fn weeks(&self) -> Duration;
    #[inline]
    fn week(&self) -> Duration {
        self.weeks()
    }

    fn fortnights(&self) -> Duration;
    #[inline]
    fn fortnight(&self) -> Duration {
        self.fortnights()
    }
}

// TODO: use macro to define this for all primitive integers
impl TimeDuration for i32 {
    #[inline]
    fn seconds(&self) -> Duration {
        Duration::seconds((*self).into())
    }

    #[inline]
    fn minutes(&self) -> Duration {
        Duration::minutes((*self).into())
    }

    #[inline]
    fn hours(&self) -> Duration {
        Duration::hours((*self).into())
    }

    #[inline]
    fn days(&self) -> Duration {
        Duration::days((*self).into())
    }

    #[inline]
    fn weeks(&self) -> Duration {
        Duration::weeks((*self).into())
    }

    #[inline]
    fn fortnights(&self) -> Duration {
        Duration::weeks((2 * self).into())
    }

    #[inline]
    fn in_milliseconds(&self) -> Duration {
        Duration::milliseconds((*self * 1000).into())
    }
    // TODO: months, years
}

pub trait TimeRange {
    // TODO: in_minutes, in_hours, in_days, in_weeks, in_months, in_years
    #[allow(clippy::wrong_self_convention)]
    fn from_now(&self) -> Option<chrono::DateTime<Utc>>;
    #[inline]
    fn since(&self) -> Option<chrono::DateTime<Utc>> {
        self.from_now()
    }

    #[inline]
    fn after(&self) -> Option<chrono::DateTime<Utc>> {
        self.from_now()
    }

    fn ago(&self) -> Option<DateTime<Utc>>;

    #[inline]
    fn until(&self) -> Option<DateTime<Utc>> {
        self.ago()
    }

    #[inline]
    fn before(&self) -> Option<DateTime<Utc>> {
        self.ago()
    }
}

impl TimeRange for Duration {
    fn from_now(&self) -> Option<DateTime<Utc>> {
        Utc::now().checked_add_signed(self.to_owned())
    }

    fn ago(&self) -> Option<DateTime<Utc>> {
        Utc::now().checked_sub_signed(self.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duration_works() {
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
    }

    #[test]
    fn range_works() {
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
}
