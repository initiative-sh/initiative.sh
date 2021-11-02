pub use command::TimeCommand;
pub use interval::Interval;

mod command;
mod interval;

use std::fmt;
use std::str::FromStr;

#[derive(Debug, Default, PartialEq)]
pub struct Time {
    days: i32,
    hours: u8,
    minutes: u8,
    seconds: u8,
}

pub struct TimeShortView<'a>(&'a Time);

pub struct TimeLongView<'a>(&'a Time);

impl Time {
    pub fn try_new(days: i32, hours: u8, minutes: u8, seconds: u8) -> Result<Self, ()> {
        if hours < 24 && minutes < 60 && seconds < 60 {
            Ok(Self {
                days,
                hours,
                minutes,
                seconds,
            })
        } else {
            Err(())
        }
    }

    pub fn checked_add(&self, interval: &Interval) -> Option<Self> {
        let (mut days, mut hours, mut minutes, mut seconds) = (
            (self.days as i64) + (interval.days as i64),
            (self.hours as i64) + (interval.hours as i64),
            (self.minutes as i64) + (interval.minutes as i64),
            (self.seconds as i64) + (interval.seconds as i64),
        );

        if interval.rounds != 0 {
            seconds += interval.rounds as i64 * 6;
        }

        if !(0..60).contains(&seconds) {
            minutes += seconds.div_euclid(60);
            seconds = seconds.rem_euclid(60);
        }

        if !(0..60).contains(&minutes) {
            hours += minutes.div_euclid(60);
            minutes = minutes.rem_euclid(60);
        }

        if !(0..24).contains(&hours) {
            days += hours.div_euclid(24);
            hours = hours.rem_euclid(24);
        }

        if let (Ok(days), Ok(hours), Ok(minutes), Ok(seconds)) = (
            days.try_into(),
            hours.try_into(),
            minutes.try_into(),
            seconds.try_into(),
        ) {
            Some(Self {
                days,
                hours,
                minutes,
                seconds,
            })
        } else {
            None
        }
    }

    pub fn checked_sub(&self, interval: &Interval) -> Option<Self> {
        if let (Some(days), Some(hours), Some(minutes), Some(seconds), Some(rounds)) = (
            0i32.checked_sub(interval.days),
            0i32.checked_sub(interval.hours),
            0i32.checked_sub(interval.minutes),
            0i32.checked_sub(interval.seconds),
            0i32.checked_sub(interval.rounds),
        ) {
            self.checked_add(&Interval {
                days,
                hours,
                minutes,
                seconds,
                rounds,
            })
        } else {
            None
        }
    }

    pub fn display_short(&self) -> TimeShortView {
        TimeShortView(self)
    }

    pub fn display_long(&self) -> TimeLongView {
        TimeLongView(self)
    }
}

impl FromStr for Time {
    type Err = ();

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let mut parts = raw.split(':');

        let days = parts.next().ok_or(())?.parse().map_err(|_| ())?;
        let hours = parts.next().ok_or(())?.parse().map_err(|_| ())?;
        let minutes = parts.next().ok_or(())?.parse().map_err(|_| ())?;
        let seconds = parts.next().ok_or(())?.parse().map_err(|_| ())?;

        if parts.next().is_none() {
            Time::try_new(days, hours, minutes, seconds)
        } else {
            Err(())
        }
    }
}

impl<'a> fmt::Display for TimeShortView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let time = self.0;
        write!(
            f,
            "{}:{:02}:{:02}:{:02}",
            time.days, time.hours, time.minutes, time.seconds
        )
    }
}

impl<'a> fmt::Display for TimeLongView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let time = self.0;

        let (hours, am_pm) = match time.hours {
            0 => (12, "am"),
            1..=11 => (time.hours, "am"),
            12 => (12, "pm"),
            _ => (time.hours % 12, "pm"),
        };

        write!(
            f,
            "day {} at {}:{:02}:{:02} {}",
            time.days, hours, time.minutes, time.seconds, am_pm
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn time_try_new_test() {
        assert_eq!(Ok(t(1, 2, 3, 4)), Time::try_new(1, 2, 3, 4));
        assert!(Time::try_new(i32::MAX, 23, 59, 59).is_ok());
        assert_eq!(Err(()), Time::try_new(0, 24, 0, 0));
        assert_eq!(Err(()), Time::try_new(0, 0, 60, 0));
        assert_eq!(Err(()), Time::try_new(0, 0, 0, 60));
    }

    #[test]
    fn time_checked_add_test() {
        assert_eq!(
            t(3, 6, 9, 12),
            t(1, 2, 3, 4)
                .checked_add(&Interval::new(2, 4, 6, 8, 0))
                .unwrap(),
        );

        assert_eq!(
            t(1, 1, 1, 1),
            Time::default()
                .checked_add(&Interval::new_seconds(86400 + 3600 + 60 + 1))
                .unwrap(),
        );

        assert_eq!(
            t(1, 1, 1, 0),
            Time::default()
                .checked_add(&Interval::new_minutes(1440 + 60 + 1))
                .unwrap(),
        );

        assert_eq!(
            t(1, 1, 0, 0),
            Time::default()
                .checked_add(&Interval::new_hours(24 + 1))
                .unwrap(),
        );

        assert_eq!(
            t(2, 0, 0, 0),
            t(1, 23, 59, 59)
                .checked_add(&Interval::new_seconds(1))
                .unwrap(),
        );

        assert_eq!(
            t(0, 0, 0, 6),
            t0().checked_add(&Interval::new_rounds(1)).unwrap(),
        );
        assert_eq!(
            t(0, 0, 1, 0),
            t0().checked_add(&Interval::new_rounds(10)).unwrap(),
        );
    }

    #[test]
    fn time_checked_add_test_limits() {
        assert!(tmax().checked_add(&Interval::new_days(1)).is_none());
        assert!(t1()
            .checked_add(&Interval::new_days(i32::MAX - 1))
            .is_some());
        assert!(t1().checked_add(&Interval::new_days(i32::MAX)).is_none());

        assert!(tmax().checked_add(&Interval::new_hours(1)).is_none());
        assert!(t1().checked_add(&Interval::new_hours(i32::MAX)).is_some());

        assert!(tmax().checked_add(&Interval::new_minutes(1)).is_none());
        assert!(t1().checked_add(&Interval::new_minutes(i32::MAX)).is_some());

        assert!(tmax().checked_add(&Interval::new_seconds(1)).is_none());
        assert!(t1().checked_add(&Interval::new_seconds(i32::MAX)).is_some());

        assert!(tmax().checked_add(&Interval::new_rounds(1)).is_none());
        assert!(t1().checked_add(&Interval::new_rounds(i32::MAX)).is_some());
    }

    #[test]
    fn time_checked_sub_test() {
        assert_eq!(
            t(-1, 0, 0, 0),
            t0().checked_sub(&Interval::new_days(1)).unwrap(),
        );

        assert_eq!(
            t0(),
            t(0, 1, 0, 0).checked_sub(&Interval::new_hours(1)).unwrap(),
        );
        assert_eq!(
            t(-1, 23, 0, 0),
            t0().checked_sub(&Interval::new_hours(1)).unwrap(),
        );

        assert_eq!(
            t0(),
            t(0, 0, 1, 0)
                .checked_sub(&Interval::new_minutes(1))
                .unwrap(),
        );
        assert_eq!(
            t(-1, 23, 59, 0),
            t0().checked_sub(&Interval::new_minutes(1)).unwrap(),
        );

        assert_eq!(
            t0(),
            t(0, 0, 0, 1)
                .checked_sub(&Interval::new_seconds(1))
                .unwrap(),
        );
        assert_eq!(
            t(-1, 23, 59, 59),
            t0().checked_sub(&Interval::new_seconds(1)).unwrap(),
        );
    }

    #[test]
    fn time_checked_sub_test_limits() {
        assert!(t0().checked_sub(&Interval::new_days(i32::MAX)).is_some());
        assert!(t0().checked_sub(&Interval::new_days(i32::MIN)).is_none());

        assert!(t0().checked_sub(&Interval::new_hours(i32::MAX)).is_some());
        assert!(t0().checked_sub(&Interval::new_hours(i32::MIN)).is_none());

        assert!(t0().checked_sub(&Interval::new_minutes(i32::MAX)).is_some());
        assert!(t0().checked_sub(&Interval::new_minutes(i32::MIN)).is_none());

        assert!(t0().checked_sub(&Interval::new_seconds(i32::MAX)).is_some());
        assert!(t0().checked_sub(&Interval::new_seconds(i32::MIN)).is_none());
    }

    #[test]
    fn time_display_short_test() {
        assert_eq!("1:02:03:04", t(1, 2, 3, 4).display_short().to_string());
        assert_eq!("1:23:59:59", t(1, 23, 59, 59).display_short().to_string());
    }

    #[test]
    fn time_display_long_test() {
        assert_eq!("day 0 at 12:00:00 am", t0().display_long().to_string());
        assert_eq!(
            "day 1 at 1:02:03 am",
            t(1, 1, 2, 3).display_long().to_string(),
        );
        assert_eq!(
            "day 2 at 11:59:59 am",
            t(2, 11, 59, 59).display_long().to_string(),
        );
        assert_eq!(
            "day 3 at 12:00:00 pm",
            t(3, 12, 0, 0).display_long().to_string(),
        );
        assert_eq!(
            "day 4 at 1:00:00 pm",
            t(4, 13, 0, 0).display_long().to_string(),
        );
        assert_eq!(
            "day 5 at 11:59:59 pm",
            t(5, 23, 59, 59).display_long().to_string(),
        );
    }

    #[test]
    fn time_from_str_test() {
        assert_eq!(Ok(t(1, 2, 3, 4)), "1:02:03:04".parse());
        assert_eq!(Ok(t(1, 23, 59, 59)), "1:23:59:59".parse());
    }

    fn t(days: i32, hours: u8, minutes: u8, seconds: u8) -> Time {
        Time {
            days,
            hours,
            minutes,
            seconds,
        }
    }

    fn t0() -> Time {
        t(0, 0, 0, 0)
    }

    fn t1() -> Time {
        t(1, 1, 1, 1)
    }

    fn tmax() -> Time {
        t(i32::MAX, 23, 59, 59)
    }
}
