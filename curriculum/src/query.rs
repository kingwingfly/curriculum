use std::{borrow::Borrow, collections::HashSet};

use paste::paste;

use crate::{course::Course, time_provider::TimeProvider};

pub trait Query<C, M>
where
    C: Borrow<Course>,
{
    fn filter<F>(self, f: F) -> impl Iterator<Item = C>
    where
        F: Fn(&Course) -> bool;

    fn in_progress<TPB, TP>(self, time_provider: TPB) -> impl Iterator<Item = C>
    where
        Self: Sized,
        TPB: Borrow<TP>,
        TP: TimeProvider,
    {
        self.filter(move |c| c.is_in_progress::<&TP, TP>(time_provider.borrow()))
    }

    fn filter_by_name<NS, N>(self, names: NS) -> impl Iterator<Item = C>
    where
        Self: Sized,
        NS: IntoIterator<Item = N>,
        N: AsRef<str>,
    {
        let names = names
            .into_iter()
            .map(|n| n.as_ref().to_owned())
            .collect::<HashSet<_>>();
        self.filter(move |c| names.contains(c.name().as_str()))
    }
}

impl<T, C> Query<C, ()> for T
where
    T: IntoIterator<Item = C>,
    C: Borrow<Course>,
{
    fn filter<F>(self, f: F) -> impl Iterator<Item = C>
    where
        F: Fn(&Course) -> bool,
    {
        Iterator::filter(self.into_iter(), move |c| f(c.borrow()))
    }
}

macro_rules! impl_query {
    ($($i: literal),+ $(,)?) => {
        paste! {
            impl<$([<T $i>]),+, C> Query<C, ((),)> for ($([<T $i>]),+,)
            where
                $([<T $i>]: IntoIterator<Item = C>),+,
                C: Borrow<Course>,
            {
                fn filter<F>(self, f: F) -> impl Iterator<Item = C>
                where
                    F: Fn(&Course) -> bool,
                {
                    Iterator::filter([].into_iter()$(.chain(self.$i))*, move |c| f(c.borrow()))
                }
            }
        }
    };
}

impl_query!(0,);
impl_query!(0, 1,);
impl_query!(0, 1, 2);
impl_query!(0, 1, 2, 3);
impl_query!(0, 1, 2, 3, 4);
impl_query!(0, 1, 2, 3, 4, 5);

#[cfg(test)]
mod tests {
    use chrono::{Days, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta};

    use super::*;
    use crate::{period::CoursePeriod, time_provider::MockTimeProvider};

    #[test]
    fn calender_test() {
        let time_provider = MockTimeProvider::new(NaiveDateTime::new(
            NaiveDate::MIN, // Thursday
            NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
        ));

        let courses = (
            [
                Course::builder()
                    .name("Chinese")
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
                    .build(),
                Course::builder()
                    .name("English")
                    .tutors([])
                    .periods(vec![
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
            ],
            [Course::builder()
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
                .build()],
        );

        assert_eq!(
            (courses.0.iter(), courses.1.iter())
                .in_progress(time_provider)
                .collect::<Vec<_>>(),
            [&courses.0[0]]
        );
        assert_eq!(
            (courses.0.iter(), courses.1.iter())
                .filter_by_name(["Math", "History"])
                .collect::<Vec<_>>(),
            [&courses.1[0]]
        );
        assert_eq!(
            (courses.0.iter(), courses.1.iter())
                .filter_by_name(["Chinese", "Math", "History"])
                .collect::<Vec<_>>(),
            [&courses.0[0], &courses.1[0]]
        );
        assert!(
            (courses.0.iter(), courses.1.iter())
                .filter_by_name(["History"])
                .next()
                .is_none()
        );
        assert_eq!(
            (courses.0.iter(), courses.1.iter())
                .filter_by_name(["Chinese", "Math", "History"])
                .in_progress(time_provider)
                .collect::<Vec<_>>(),
            [&courses.0[0]]
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
