use std::{borrow::Borrow, collections::HashSet};

use crate::{
    course::{Course, CoursePeriod},
    time_provider::TimeProvider,
};

pub trait Query<C, PS>: Sized
where
    for<'a> &'a PS: IntoIterator<Item = &'a CoursePeriod>,
    C: Borrow<Course<PS>>,
    Self: IntoIterator<Item = C>,
{
    fn filter<F>(self, f: F) -> impl Iterator<Item = C>
    where
        F: Fn(&Course<PS>) -> bool,
    {
        Iterator::filter(self.into_iter(), move |c| f(c.borrow()))
    }

    fn in_progress<TPB, TP>(self, time_provider: TPB) -> impl Iterator<Item = C>
    where
        TPB: Borrow<TP>,
        TP: TimeProvider,
    {
        self.filter(move |c| c.is_in_progress::<&TP, TP>(time_provider.borrow()))
    }

    fn filter_by_name<NS, N>(self, names: NS) -> impl Iterator<Item = C>
    where
        NS: IntoIterator<Item = N>,
        N: Borrow<str>,
    {
        let names = names
            .into_iter()
            .map(|n| n.borrow().to_owned())
            .collect::<HashSet<_>>();
        self.filter(move |c| names.contains(c.name().as_str()))
    }
}

impl<T, C, PS> Query<C, PS> for T
where
    for<'a> &'a PS: IntoIterator<Item = &'a CoursePeriod>,
    C: Borrow<Course<PS>>,
    Self: IntoIterator<Item = C>,
{
}

#[cfg(test)]
mod tests {
    use chrono::{Days, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta};

    use super::*;
    use crate::{course::CoursePeriod, time_provider::MockTimeProvider};

    #[test]
    fn calender_test() {
        let time_provider = MockTimeProvider::new(NaiveDateTime::new(
            NaiveDate::MIN, // Thursday
            NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
        ));

        let courses = vec![
            Course::builder()
                .name("Chinese")
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
                .build(),
            Course::builder()
                .name("English")
                .tutors([])
                .periods([
                    CoursePeriod::builder()
                        .start(NaiveDate::MIN.and_hms_opt(14, 0, 0).unwrap())
                        .duration(TimeDelta::hours(4))
                        .build(),
                    CoursePeriod::builder()
                        .start(
                            NaiveDate::MIN
                                .checked_add_days(Days::new(1))
                                .unwrap()
                                .and_hms_opt(14, 0, 0)
                                .unwrap(),
                        )
                        .duration(TimeDelta::hours(4))
                        .build(),
                ])
                .build(),
            Course::builder()
                .name("Math")
                .tutors([])
                .periods([
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
                    CoursePeriod::builder()
                        .start(
                            NaiveDate::MIN
                                .checked_add_days(Days::new(3))
                                .unwrap()
                                .and_hms_opt(8, 0, 0)
                                .unwrap(),
                        )
                        .duration(TimeDelta::hours(4))
                        .build(),
                ])
                .build(),
        ];

        assert_eq!(
            courses
                .iter()
                .in_progress(time_provider)
                .collect::<Vec<_>>(),
            [&courses[0]]
        );
        assert_eq!(
            courses
                .iter()
                .filter_by_name(["Math", "History"])
                .collect::<Vec<_>>(),
            [&courses[2]]
        );
        assert_eq!(
            courses
                .iter()
                .filter_by_name(["Chinese", "Math", "History"])
                .collect::<Vec<_>>(),
            [&courses[0], &courses[2]]
        );
        assert!(courses.iter().filter_by_name(["History"]).next().is_none());
        assert_eq!(
            courses
                .iter()
                .filter_by_name(["Chinese", "Math", "History"])
                .in_progress(time_provider)
                .collect::<Vec<_>>(),
            [&courses[0]]
        );
        assert!(
            courses
                .filter_by_name(["English", "Math", "History"])
                .in_progress(time_provider)
                .next()
                .is_none()
        );
    }
}
