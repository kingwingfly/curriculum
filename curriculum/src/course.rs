use std::{borrow::Borrow, collections::HashSet};

use bon::bon;
use chrono::{NaiveDateTime, TimeDelta};
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
    /// Local
    #[getset(get = "pub")]
    start: NaiveDateTime,
    /// Local
    #[getset(get = "pub")]
    end: NaiveDateTime,
    #[getset(get = "pub")]
    duration: TimeDelta,
}

#[bon]
impl CoursePeriod {
    #[builder]
    pub fn new(start: NaiveDateTime, duration: TimeDelta) -> Self {
        Self {
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
        self.periods.iter().any(|p| p.start <= now && now <= p.end)
    }
}

#[cfg(test)]
mod test {
    use chrono::{Days, NaiveDate};

    use super::*;
    use crate::time_provider::MockTimeProvider;

    #[test]
    fn course_tests() {
        let time_provider = MockTimeProvider::new(NaiveDate::MIN.and_hms_opt(8, 0, 0).unwrap());

        let course = Course::builder()
            .name("English")
            .tutors([])
            .periods([CoursePeriod::builder()
                .start(NaiveDate::MIN.and_hms_opt(8, 0, 0).unwrap())
                .duration(TimeDelta::hours(4))
                .build()])
            .build();
        assert!(course.is_in_progress(time_provider));

        let course = Course::builder()
            .name("Chinese")
            .tutors([])
            .periods([CoursePeriod::builder()
                .start(NaiveDate::MIN.and_hms_opt(14, 0, 0).unwrap())
                .duration(TimeDelta::hours(4))
                .build()])
            .build();
        assert!(!course.is_in_progress(time_provider));

        let course = Course::builder()
            .name("Math")
            .tutors([])
            .periods([CoursePeriod::builder()
                .start(
                    NaiveDate::MIN
                        .checked_add_days(Days::new(1))
                        .unwrap()
                        .and_hms_opt(8, 0, 0)
                        .unwrap(),
                )
                .duration(TimeDelta::hours(4))
                .build()])
            .build();
        assert!(!course.is_in_progress(time_provider));

        let course = Course::builder()
            .name("History")
            .tutors([])
            .periods([
                CoursePeriod::builder()
                    .start(NaiveDate::MIN.and_hms_opt(8, 0, 0).unwrap())
                    .duration(TimeDelta::hours(4))
                    .build(),
                CoursePeriod::builder()
                    .start(
                        NaiveDate::MIN
                            .checked_add_days(Days::new(1))
                            .unwrap()
                            .and_hms_opt(8, 0, 0)
                            .unwrap(),
                    )
                    .duration(TimeDelta::hours(4))
                    .build(),
            ])
            .build();
        assert!(course.is_in_progress(time_provider));
    }
}
