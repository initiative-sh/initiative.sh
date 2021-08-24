use std::convert::TryInto;
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

#[derive(Debug, PartialEq)]
pub struct Interval {
    days: i32,
    hours: i32,
    minutes: i32,
    seconds: i32,
    rounds: i32,
}

impl Time {
    #[allow(dead_code)]
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

    #[allow(dead_code)]
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

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn display_short(&self) -> TimeShortView {
        TimeShortView(self)
    }

    #[allow(dead_code)]
    pub fn display_long(&self) -> TimeLongView {
        TimeLongView(self)
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

impl Interval {
    pub fn new(days: i32, hours: i32, minutes: i32, seconds: i32, rounds: i32) -> Self {
        Self {
            days,
            hours,
            minutes,
            seconds,
            rounds,
        }
    }

    pub fn days(days: i32) -> Self {
        Self::new(days, 0, 0, 0, 0)
    }

    pub fn hours(hours: i32) -> Self {
        Self::new(0, hours, 0, 0, 0)
    }

    pub fn minutes(minutes: i32) -> Self {
        Self::new(0, 0, minutes, 0, 0)
    }

    pub fn seconds(seconds: i32) -> Self {
        Self::new(0, 0, 0, seconds, 0)
    }

    pub fn rounds(rounds: i32) -> Self {
        Self::new(0, 0, 0, 0, rounds)
    }
}

impl FromStr for Interval {
    type Err = ();

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if let Some((i, c)) = raw.char_indices().last() {
            let value = if i == 0 {
                // Interpret input like "d" as "1d"
                1
            } else if raw.starts_with(|c: char| c.is_ascii_digit()) {
                raw[..i].parse().map_err(|_| ())?
            } else {
                // Don't accept "-1d", that's handled by the command parser
                return Err(());
            };

            match c {
                'd' => Ok(Self::days(value)),
                'h' => Ok(Self::hours(value)),
                'm' => Ok(Self::minutes(value)),
                's' => Ok(Self::seconds(value)),
                'r' => Ok(Self::rounds(value)),
                _ => Err(()),
            }
        } else {
            Err(())
        }
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;

        let mut w = |value: i32, name: &str| -> fmt::Result {
            if value != 0 {
                if !first {
                    write!(f, ", ")?;
                }

                write!(f, "{} {}{}", value, name, if value == 1 { "" } else { "s" })?;
                first = false;
            }

            Ok(())
        };

        w(self.days, "day")?;
        w(self.hours, "hour")?;
        w(self.minutes, "minute")?;
        w(self.seconds, "second")?;
        w(self.rounds, "round")?;

        Ok(())
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
            t(1, 2, 3, 4).checked_add(&i(2, 4, 6, 8, 0)).unwrap(),
        );

        assert_eq!(
            t(1, 1, 1, 1),
            Time::default()
                .checked_add(&seconds(86400 + 3600 + 60 + 1))
                .unwrap(),
        );

        assert_eq!(
            t(1, 1, 1, 0),
            Time::default()
                .checked_add(&minutes(1440 + 60 + 1))
                .unwrap(),
        );

        assert_eq!(
            t(1, 1, 0, 0),
            Time::default().checked_add(&hours(24 + 1)).unwrap(),
        );

        assert_eq!(
            t(2, 0, 0, 0),
            t(1, 23, 59, 59).checked_add(&seconds(1)).unwrap(),
        );

        assert_eq!(t(0, 0, 0, 6), t0().checked_add(&rounds(1)).unwrap());
        assert_eq!(t(0, 0, 1, 0), t0().checked_add(&rounds(10)).unwrap());
    }

    #[test]
    fn time_checked_add_test_limits() {
        assert!(tmax().checked_add(&days(1)).is_none());
        assert!(t1().checked_add(&days(i32::MAX - 1)).is_some());
        assert!(t1().checked_add(&days(i32::MAX)).is_none());

        assert!(tmax().checked_add(&hours(1)).is_none());
        assert!(t1().checked_add(&hours(i32::MAX)).is_some());

        assert!(tmax().checked_add(&minutes(1)).is_none());
        assert!(t1().checked_add(&minutes(i32::MAX)).is_some());

        assert!(tmax().checked_add(&seconds(1)).is_none());
        assert!(t1().checked_add(&seconds(i32::MAX)).is_some());

        assert!(tmax().checked_add(&rounds(1)).is_none());
        assert!(t1().checked_add(&rounds(i32::MAX)).is_some());
    }

    #[test]
    fn time_checked_sub_test() {
        assert_eq!(t(-1, 0, 0, 0), t0().checked_sub(&days(1)).unwrap());

        assert_eq!(t0(), t(0, 1, 0, 0).checked_sub(&hours(1)).unwrap());
        assert_eq!(t(-1, 23, 0, 0), t0().checked_sub(&hours(1)).unwrap());

        assert_eq!(t0(), t(0, 0, 1, 0).checked_sub(&minutes(1)).unwrap());
        assert_eq!(t(-1, 23, 59, 0), t0().checked_sub(&minutes(1)).unwrap());

        assert_eq!(t0(), t(0, 0, 0, 1).checked_sub(&seconds(1)).unwrap());
        assert_eq!(t(-1, 23, 59, 59), t0().checked_sub(&seconds(1)).unwrap());
    }

    #[test]
    fn time_checked_sub_test_limits() {
        assert!(t0().checked_sub(&days(i32::MAX)).is_some());
        assert!(t0().checked_sub(&days(i32::MIN)).is_none());

        assert!(t0().checked_sub(&hours(i32::MAX)).is_some());
        assert!(t0().checked_sub(&hours(i32::MIN)).is_none());

        assert!(t0().checked_sub(&minutes(i32::MAX)).is_some());
        assert!(t0().checked_sub(&minutes(i32::MIN)).is_none());

        assert!(t0().checked_sub(&seconds(i32::MAX)).is_some());
        assert!(t0().checked_sub(&seconds(i32::MIN)).is_none());
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
    fn interval_new_test() {
        assert_eq!(
            i(100, 200, 300, 400, 500),
            Interval::new(100, 200, 300, 400, 500),
        );

        assert_eq!(i(1, 0, 0, 0, 0), Interval::days(1));
        assert_eq!(i(0, 1, 0, 0, 0), Interval::hours(1));
        assert_eq!(i(0, 0, 1, 0, 0), Interval::minutes(1));
        assert_eq!(i(0, 0, 0, 1, 0), Interval::seconds(1));
        assert_eq!(i(0, 0, 0, 0, 1), Interval::rounds(1));
    }

    #[test]
    fn interval_from_str_test() {
        assert_eq!(Ok(days(10)), "10d".parse());
        assert_eq!(Ok(hours(10)), "10h".parse());
        assert_eq!(Ok(minutes(10)), "10m".parse());
        assert_eq!(Ok(seconds(10)), "10s".parse());
        assert_eq!(Ok(rounds(10)), "10r".parse());

        assert_eq!(Ok(days(1)), "d".parse());
        assert_eq!(Ok(hours(1)), "h".parse());
        assert_eq!(Ok(minutes(1)), "m".parse());
        assert_eq!(Ok(seconds(1)), "s".parse());
        assert_eq!(Ok(rounds(1)), "r".parse());

        assert_eq!(Ok(days(0)), "0d".parse());
        assert_eq!(Ok(days(1)), "01d".parse());
        assert_eq!(Ok(days(i32::MAX)), format!("{}d", i32::MAX).parse());

        assert_eq!(Err(()), format!("{}d", i64::MAX).parse::<Interval>());
        assert_eq!(Err(()), "".parse::<Interval>());
        assert_eq!(Err(()), "1 d".parse::<Interval>());
        assert_eq!(Err(()), "1a".parse::<Interval>());
        assert_eq!(Err(()), "-1d".parse::<Interval>());
    }

    #[test]
    fn interval_display_test() {
        assert_eq!("1 day", days(1).to_string());
        assert_eq!("1 hour", hours(1).to_string());
        assert_eq!("1 minute", minutes(1).to_string());
        assert_eq!("1 second", seconds(1).to_string());
        assert_eq!("1 round", rounds(1).to_string());

        assert_eq!(
            "2 days, 3 hours, 4 minutes, 5 seconds, 6 rounds",
            i(2, 3, 4, 5, 6).to_string(),
        );
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

    fn i(days: i32, hours: i32, minutes: i32, seconds: i32, rounds: i32) -> Interval {
        Interval {
            days,
            hours,
            minutes,
            seconds,
            rounds,
        }
    }

    fn days(days: i32) -> Interval {
        i(days, 0, 0, 0, 0)
    }

    fn hours(hours: i32) -> Interval {
        i(0, hours, 0, 0, 0)
    }

    fn minutes(minutes: i32) -> Interval {
        i(0, 0, minutes, 0, 0)
    }

    fn seconds(seconds: i32) -> Interval {
        i(0, 0, 0, seconds, 0)
    }

    fn rounds(rounds: i32) -> Interval {
        i(0, 0, 0, 0, rounds)
    }
}
