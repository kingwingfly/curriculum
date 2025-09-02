use std::{borrow::Borrow, collections::HashSet, fmt};

use bon::bon;
use chrono::TimeDelta;
use getset::Getters;

use crate::{period::Period, time_provider::TimeProvider, tutor::Tutor};

#[derive(Debug, PartialEq, Eq, Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Course {
    #[getset(get = "pub")]
    name: String,
    #[getset(get = "pub")]
    place: Option<String>,
    #[getset(get = "pub")]
    tutors: HashSet<Tutor>,
    #[getset(get = "pub")]
    periods: HashSet<Period>,
}

impl fmt::Display for Course {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}) <{}>",
            self.name,
            self.place.as_deref().unwrap_or("Unknown"),
            self.tutors.iter().fold(String::new(), |mut acc, tutor| {
                if !acc.is_empty() {
                    acc.push(' ');
                }
                acc.push_str(tutor.name());
                acc
            })
        )?;
        Ok(())
    }
}

#[bon]
impl Course {
    #[builder]
    pub fn new(
        name: &str,
        place: Option<&str>,
        tutors: impl IntoIterator<Item = Tutor>,
        periods: impl IntoIterator<Item = Period>,
    ) -> Self {
        Self {
            name: name.to_owned(),
            place: place.map(|s| s.to_owned()),
            tutors: tutors.into_iter().collect(),
            periods: periods.into_iter().collect(),
        }
    }
}

impl Course {
    pub fn is_in_progress<TP: TimeProvider>(&self, time_provider: impl Borrow<TP>) -> bool {
        let now = time_provider.borrow().now();
        self.periods
            .iter()
            .any(|p| p.start() <= &now && &now <= p.end())
    }

    pub fn is_upcoming<TP: TimeProvider>(
        &self,
        time_provider: impl Borrow<TP>,
        delta: TimeDelta,
    ) -> bool {
        let start = time_provider.borrow().now();
        let end = start + delta;
        self.periods
            .iter()
            .any(|p| &start <= p.start() && p.start() <= &end)
    }

    pub fn nearest_period<TP: TimeProvider>(
        &self,
        time_provider: impl Borrow<TP>,
    ) -> Option<&Period> {
        let now = time_provider.borrow().now();
        self.periods
            .iter()
            .filter(|p| p.start() > &now)
            .min_by_key(|p| p.start())
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
            .periods([Period::builder()
                .start(NaiveDate::MIN.and_hms_opt(8, 0, 0).unwrap())
                .duration(TimeDelta::hours(4))
                .build()])
            .build();
        assert!(course.is_in_progress(time_provider));

        let course = Course::builder()
            .name("Chinese")
            .tutors([])
            .periods([Period::builder()
                .start(NaiveDate::MIN.and_hms_opt(14, 0, 0).unwrap())
                .duration(TimeDelta::hours(4))
                .build()])
            .build();
        assert!(!course.is_in_progress(time_provider));

        let course = Course::builder()
            .name("Math")
            .tutors([])
            .periods([Period::builder()
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
                Period::builder()
                    .start(NaiveDate::MIN.and_hms_opt(8, 0, 0).unwrap())
                    .duration(TimeDelta::hours(4))
                    .build(),
                Period::builder()
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
                        Period::builder()
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
