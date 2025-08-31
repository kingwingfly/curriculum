use std::{borrow::Borrow, collections::HashSet};

use bon::bon;
use chrono::{Datelike as _, NaiveTime, TimeDelta, Weekday};
use getset::Getters;

use crate::{time_provider::TimeProvider, tutor::Tutor};

#[derive(Debug, Getters, PartialEq, Eq)]
pub struct Course {
    #[getset(get = "pub")]
    name: String,
    #[getset(get = "pub")]
    tutors: HashSet<Tutor>,
    #[getset(get = "pub")]
    periods: HashSet<CoursePeriod>,
}

#[bon]
impl Course {
    #[builder]
    pub fn new(
        name: impl AsRef<str>,
        tutors: impl Into<HashSet<Tutor>>,
        periods: impl Into<HashSet<CoursePeriod>>,
    ) -> Self {
        Self {
            name: name.as_ref().to_owned(),
            tutors: tutors.into(),
            periods: periods.into(),
        }
    }
}

#[derive(Debug, Getters, PartialEq, Eq, Clone, Copy, Hash)]
pub struct CoursePeriod {
    #[getset(get = "pub")]
    day: Weekday,
    /// Local
    #[getset(get = "pub")]
    start: NaiveTime,
    /// Local
    #[getset(get = "pub")]
    end: NaiveTime,
    #[getset(get = "pub")]
    duration: TimeDelta,
}

#[bon]
impl CoursePeriod {
    #[builder]
    pub fn new(day: Weekday, start: NaiveTime, duration: TimeDelta) -> Self {
        Self {
            day,
            start,
            end: start + duration,
            duration,
        }
    }
}

impl Course {
    pub fn is_in_progress<TPB, TP>(&self, time_provider: TPB) -> bool
    where
        TPB: Borrow<TP>,
        TP: TimeProvider,
    {
        let now = time_provider.borrow().now();
        let day = now.weekday();
        let time = now.time();
        self.periods
            .iter()
            .any(|p| day == p.day && p.start <= time && time <= p.end)
    }
}

#[cfg(test)]
mod test {
    use chrono::{NaiveDate, NaiveDateTime};

    use super::*;
    use crate::time_provider::MockTimeProvider;

    #[test]
    fn course_tests() {
        let time_provider = MockTimeProvider::new(NaiveDateTime::new(
            NaiveDate::MIN, // Thursday
            NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
        ));

        let course = Course::builder()
            .name("English")
            .tutors([])
            .periods([CoursePeriod::builder()
                .day(Weekday::Thu)
                .start(NaiveTime::from_hms_opt(8, 0, 0).unwrap())
                .duration(TimeDelta::hours(4))
                .build()])
            .build();
        assert!(course.is_in_progress(time_provider));

        let course = Course::builder()
            .name("Chinese")
            .tutors([])
            .periods([CoursePeriod::builder()
                .day(Weekday::Thu)
                .start(NaiveTime::from_hms_opt(14, 0, 0).unwrap())
                .duration(TimeDelta::hours(4))
                .build()])
            .build();
        assert!(!course.is_in_progress(time_provider));

        let course = Course::builder()
            .name("Math")
            .tutors([])
            .periods([CoursePeriod::builder()
                .day(Weekday::Sun)
                .start(NaiveTime::from_hms_opt(8, 0, 0).unwrap())
                .duration(TimeDelta::hours(4))
                .build()])
            .build();
        assert!(!course.is_in_progress(time_provider));

        let course = Course::builder()
            .name("History")
            .tutors([])
            .periods([
                CoursePeriod::builder()
                    .day(Weekday::Mon)
                    .start(NaiveTime::from_hms_opt(8, 0, 0).unwrap())
                    .duration(TimeDelta::hours(4))
                    .build(),
                CoursePeriod::builder()
                    .day(Weekday::Thu)
                    .start(NaiveTime::from_hms_opt(8, 0, 0).unwrap())
                    .duration(TimeDelta::hours(4))
                    .build(),
            ])
            .build();
        assert!(course.is_in_progress(time_provider));
    }
}
