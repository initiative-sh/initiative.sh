use std::collections::HashSet;
use std::fmt;
use std::ops::AddAssign;
use std::str::FromStr;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Interval {
    pub days: i32,
    pub hours: i32,
    pub minutes: i32,
    pub seconds: i32,
    pub rounds: i32,
}

pub struct IntervalShortView<'a>(&'a Interval);

pub struct IntervalLongView<'a>(&'a Interval);

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

    pub fn new_days(days: i32) -> Self {
        Self::new(days, 0, 0, 0, 0)
    }

    pub fn new_hours(hours: i32) -> Self {
        Self::new(0, hours, 0, 0, 0)
    }

    pub fn new_minutes(minutes: i32) -> Self {
        Self::new(0, 0, minutes, 0, 0)
    }

    pub fn new_seconds(seconds: i32) -> Self {
        Self::new(0, 0, 0, seconds, 0)
    }

    pub fn new_rounds(rounds: i32) -> Self {
        Self::new(0, 0, 0, 0, rounds)
    }

    pub fn display_short(&self) -> IntervalShortView {
        IntervalShortView(self)
    }

    pub fn display_long(&self) -> IntervalLongView {
        IntervalLongView(self)
    }
}

impl AddAssign for Interval {
    fn add_assign(&mut self, other: Self) {
        self.days += other.days;
        self.hours += other.hours;
        self.minutes += other.minutes;
        self.seconds += other.seconds;
        self.rounds += other.rounds;
    }
}

impl FromStr for Interval {
    type Err = ();

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        match raw.trim() {
            "" => Err(()),
            "0" => Ok(Interval::default()),
            s => {
                let mut used_chars = HashSet::new();
                let mut interval = Interval::default();

                s.split_inclusive(|c: char| !c.is_ascii_digit())
                    .enumerate()
                    .try_for_each(|(raw_index, s)| {
                        let part = s.trim();

                        if part.is_empty() {
                            Ok(())
                        } else if let Some((part_index, c)) = part.char_indices().last() {
                            if !used_chars.insert(c.to_ascii_lowercase()) {
                                return Err(());
                            }

                            let value = if part_index == 0 && raw_index == 0 {
                                // Interpret input like "d" as "1d"
                                1
                            } else if part.starts_with(|c: char| c.is_ascii_digit()) {
                                part[..part_index].parse().map_err(|_| ())?
                            } else {
                                // Don't accept "-1d", that's handled by the command parser
                                return Err(());
                            };

                            match c {
                                'd' | 'D' => interval += Self::new_days(value),
                                'h' | 'H' => interval += Self::new_hours(value),
                                'm' | 'M' => interval += Self::new_minutes(value),
                                's' | 'S' => interval += Self::new_seconds(value),
                                'r' | 'R' => interval += Self::new_rounds(value),
                                _ => return Err(()),
                            }

                            Ok(())
                        } else {
                            Err(())
                        }
                    })?;

                Ok(interval)
            }
        }
    }
}

impl<'a> fmt::Display for IntervalShortView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let interval = self.0;
        let mut output = false;

        [
            (interval.days, 'd'),
            (interval.hours, 'h'),
            (interval.minutes, 'm'),
            (interval.seconds, 's'),
            (interval.rounds, 'r'),
        ]
        .iter()
        .filter(|(value, _)| value > &0)
        .try_for_each(|(value, name)| {
            if output {
                write!(f, " ")?;
            } else {
                output = true;
            }

            write!(f, "{}{}", value, name)
        })?;

        if !output {
            write!(f, "0")?;
        }

        Ok(())
    }
}

impl<'a> fmt::Display for IntervalLongView<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let interval = self.0;
        let mut output = false;

        [
            (interval.days, "day"),
            (interval.hours, "hour"),
            (interval.minutes, "minute"),
            (interval.seconds, "second"),
            (interval.rounds, "round"),
        ]
        .iter()
        .filter(|(value, _)| value > &0)
        .try_for_each(|(value, name)| {
            if output {
                write!(f, ", ")?;
            } else {
                output = true;
            }

            write!(
                f,
                "{} {}{}",
                value,
                name,
                if value == &1 { "" } else { "s" }
            )
        })?;

        if !output {
            write!(f, "nuthin'")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn interval_new_test() {
        assert_eq!(
            i(100, 200, 300, 400, 500),
            Interval::new(100, 200, 300, 400, 500),
        );

        assert_eq!(i(1, 0, 0, 0, 0), Interval::new_days(1));
        assert_eq!(i(0, 1, 0, 0, 0), Interval::new_hours(1));
        assert_eq!(i(0, 0, 1, 0, 0), Interval::new_minutes(1));
        assert_eq!(i(0, 0, 0, 1, 0), Interval::new_seconds(1));
        assert_eq!(i(0, 0, 0, 0, 1), Interval::new_rounds(1));
    }

    #[test]
    fn interval_from_str_test() {
        assert_eq!(Ok(days(10)), "10d".parse());
        assert_eq!(Ok(hours(10)), "10h".parse());
        assert_eq!(Ok(minutes(10)), "10m".parse());
        assert_eq!(Ok(seconds(10)), "10s".parse());
        assert_eq!(Ok(rounds(10)), "10r".parse());

        assert_eq!(Ok(days(10)), "10D".parse());
        assert_eq!(Ok(hours(10)), "10H".parse());
        assert_eq!(Ok(minutes(10)), "10M".parse());
        assert_eq!(Ok(seconds(10)), "10S".parse());
        assert_eq!(Ok(rounds(10)), "10R".parse());

        assert_eq!(Ok(days(1)), "d".parse());
        assert_eq!(Ok(hours(1)), "h".parse());
        assert_eq!(Ok(minutes(1)), "m".parse());
        assert_eq!(Ok(seconds(1)), "s".parse());
        assert_eq!(Ok(rounds(1)), "r".parse());

        assert_eq!(Ok(days(1)), "D".parse());
        assert_eq!(Ok(hours(1)), "H".parse());
        assert_eq!(Ok(minutes(1)), "M".parse());
        assert_eq!(Ok(seconds(1)), "S".parse());
        assert_eq!(Ok(rounds(1)), "R".parse());

        assert_eq!(Ok(days(0)), "0d".parse());
        assert_eq!(Ok(days(1)), "01d".parse());
        assert_eq!(Ok(days(i32::MAX)), format!("{}d", i32::MAX).parse());

        assert_eq!(Ok(Interval::default()), "0".parse());
        assert_eq!(Ok(i(2, 3, 4, 5, 6)), "2d3h4m5s6r".parse());
        assert_eq!(Ok(i(2, 3, 4, 5, 6)), "2d 3h 4m 5s 6r".parse());

        assert_eq!(Err(()), format!("{}d", i64::MAX).parse::<Interval>());
        assert_eq!(Err(()), "".parse::<Interval>());
        assert_eq!(Err(()), "1 d".parse::<Interval>());
        assert_eq!(Err(()), "1a".parse::<Interval>());
        assert_eq!(Err(()), "-1d".parse::<Interval>());
        assert_eq!(Err(()), "2d3h4m5s6r7p".parse::<Interval>());
        assert_eq!(Err(()), "1dd".parse::<Interval>());
        assert_eq!(Err(()), "2d1d".parse::<Interval>());
    }

    #[test]
    fn interval_display_short_test() {
        assert_eq!("1d", days(1).display_short().to_string());
        assert_eq!("1h", hours(1).display_short().to_string());
        assert_eq!("1m", minutes(1).display_short().to_string());
        assert_eq!("1s", seconds(1).display_short().to_string());
        assert_eq!("1r", rounds(1).display_short().to_string());

        assert_eq!("0", Interval::default().display_short().to_string());
        assert_eq!(
            "2d 3h 4m 5s 6r",
            i(2, 3, 4, 5, 6).display_short().to_string(),
        );
    }

    #[test]
    fn interval_display_long_test() {
        assert_eq!("1 day", days(1).display_long().to_string());
        assert_eq!("1 hour", hours(1).display_long().to_string());
        assert_eq!("1 minute", minutes(1).display_long().to_string());
        assert_eq!("1 second", seconds(1).display_long().to_string());
        assert_eq!("1 round", rounds(1).display_long().to_string());

        assert_eq!("nuthin'", Interval::default().display_long().to_string());

        assert_eq!(
            "2 days, 3 hours, 4 minutes, 5 seconds, 6 rounds",
            i(2, 3, 4, 5, 6).display_long().to_string(),
        );
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
