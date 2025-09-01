use std::{borrow::Borrow, collections::HashSet};

use bon::bon;
use getset::Getters;

use crate::{period::CoursePeriod, time_provider::TimeProvider, tutor::Tutor};

#[derive(Debug, PartialEq, Eq, Getters)]
pub struct Course {
    #[getset(get = "pub")]
    name: String,
    #[getset(get = "pub")]
    tutors: HashSet<Tutor>,
    #[getset(get = "pub")]
    periods: Vec<CoursePeriod>,
}

#[bon]
impl Course {
    #[builder]
    pub fn new(
        name: impl AsRef<str>,
        tutors: impl Into<HashSet<Tutor>>,
        periods: Vec<CoursePeriod>,
    ) -> Self {
        Self {
            name: name.as_ref().to_owned(),
            tutors: tutors.into(),
            periods,
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
        self.periods.iter().any(|p| p.contains(now))
    }
}

#[cfg(test)]
mod test {
    use chrono::{Days, NaiveDate, TimeDelta};

    use super::*;
    use crate::time_provider::MockTimeProvider;

    #[test]
    fn course_tests() {
        let time_provider = MockTimeProvider::new(NaiveDate::MIN.and_hms_opt(8, 0, 0).unwrap());

        let course = Course::builder()
            .name("English")
            .tutors([])
            .periods(vec![
                CoursePeriod::builder()
                    .start(NaiveDate::MIN.and_hms_opt(8, 0, 0).unwrap())
                    .duration(TimeDelta::hours(4))
                    .build(),
            ])
            .build();
        assert!(course.is_in_progress(time_provider));

        let course = Course::builder()
            .name("Chinese")
            .tutors([])
            .periods(vec![
                CoursePeriod::builder()
                    .start(NaiveDate::MIN.and_hms_opt(14, 0, 0).unwrap())
                    .duration(TimeDelta::hours(4))
                    .build(),
            ])
            .build();
        assert!(!course.is_in_progress(time_provider));

        let course = Course::builder()
            .name("Math")
            .tutors([])
            .periods(vec![
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
        assert!(!course.is_in_progress(time_provider));

        let course = Course::builder()
            .name("History")
            .tutors([])
            .periods(vec![
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

        let course = Course::builder()
            .name("Chemistry")
            .tutors([])
            .periods(
                (0..16)
                    .filter_map(|i| NaiveDate::MIN.checked_add_days(Days::new(i * 7)))
                    .filter_map(|d| d.and_hms_opt(8, 0, 0))
                    .map(|ts| {
                        CoursePeriod::builder()
                            .start(ts)
                            .duration(TimeDelta::hours(4))
                            .build()
                    })
                    .collect::<Vec<_>>(),
            )
            .build();
        assert!(course.is_in_progress(time_provider));
    }
}
