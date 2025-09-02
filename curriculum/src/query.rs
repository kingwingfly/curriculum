use std::{borrow::Borrow, collections::HashSet};

use chrono::TimeDelta;
use variadics_please::all_tuples_enumerated;

use crate::{course::Course, time_provider::TimeProvider};

pub trait Query<C, M>: Sized
where
    C: Borrow<Course>,
{
    fn into_iter(self) -> impl Iterator<Item = C>;

    fn filter<F>(self, f: F) -> impl Iterator<Item = C>
    where
        F: Fn(&Course) -> bool,
    {
        Iterator::filter(self.into_iter(), move |c| f(c.borrow()))
    }

    /// Filters the courses that are currently in progress.
    fn in_progress<TP>(self, time_provider: impl Borrow<TP>) -> impl Iterator<Item = C>
    where
        TP: TimeProvider,
    {
        self.filter(move |c| c.is_in_progress::<TP>(time_provider.borrow()))
    }

    /// Filters the courses that are upcoming in less than the specified time delta.
    fn upcoming<TP>(
        self,
        time_provider: impl Borrow<TP>,
        delta: TimeDelta,
    ) -> impl Iterator<Item = C>
    where
        TP: TimeProvider,
    {
        self.filter(move |c| c.is_upcoming::<TP>(time_provider.borrow(), delta))
    }

    fn filter_by_name<NS, N>(self, names: NS) -> impl Iterator<Item = C>
    where
        NS: IntoIterator<Item = N>,
        N: AsRef<str>,
    {
        let names = names
            .into_iter()
            .map(|n| n.as_ref().to_owned())
            .collect::<HashSet<_>>();
        self.filter(move |c| names.contains(c.name().as_str()))
    }

    fn nearest<TP>(self, time_provider: impl Borrow<TP>, num: usize) -> Vec<C>
    where
        TP: TimeProvider,
    {
        let now = time_provider.borrow().now();
        let mut courses = vec![];
        for c in self.into_iter() {
            if let Some(p) = c
                .borrow()
                .periods()
                .iter()
                .filter(|p| p.start() >= &now)
                .min_by_key(|p| p.start())
            {
                courses.push((*p.start(), c));
            }
        }
        courses.sort_unstable_by_key(|(start, _)| *start);
        let mut i = 0;
        for (start, _) in courses.iter() {
            if i >= courses.len() || (i >= num && start > &courses[i].0) {
                break;
            }
            i += 1;
        }
        courses.into_iter().take(i).map(|(_, c)| c).collect()
    }
}

impl<T, C> Query<C, ()> for T
where
    T: IntoIterator<Item = C>,
    C: Borrow<Course>,
{
    fn into_iter(self) -> impl Iterator<Item = C> {
        IntoIterator::into_iter(self)
    }
}

macro impl_query {
    ($(($i: tt, $T: ident)),+ $(,)?) => {
        impl<$($T),+, C> Query<C, ((),)> for ($($T),+,)
        where
            $($T: IntoIterator<Item = C>),+,
            C: Borrow<Course>,
        {
            fn into_iter(self) -> impl Iterator<Item = C> {
                IntoIterator::into_iter([])$(.chain(self.$i))*
            }
        }
    }
}

all_tuples_enumerated!(impl_query, 1, 9, T);

#[cfg(test)]
mod tests {
    use chrono::{Days, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta};

    use super::*;
    use crate::{period::Period, time_provider::MockTimeProvider};

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
                    .build(),
                Course::builder()
                    .name("English")
                    .tutors([])
                    .periods([
                        Period::builder()
                            .start(NaiveDate::MIN.and_hms_opt(14, 0, 0).unwrap())
                            .duration(TimeDelta::hours(4))
                            .build(),
                        Period::builder()
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
                .periods([
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
                    Period::builder()
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
