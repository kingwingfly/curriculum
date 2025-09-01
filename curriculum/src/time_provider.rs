use chrono::{Local, NaiveDateTime};

use crate::period::Holiday;

pub trait TimeProvider {
    fn now(&self) -> NaiveDateTime;

    fn is_on_holiday(&self, holidays: impl IntoIterator<Item = Holiday>) -> bool {
        let now = self.now();
        holidays
            .into_iter()
            .any(|h| h.start() <= &now && &now < h.end())
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultTimeProvider;

impl TimeProvider for DefaultTimeProvider {
    fn now(&self) -> NaiveDateTime {
        Local::now().naive_local()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MockTimeProvider {
    fixed_time: NaiveDateTime,
}

impl MockTimeProvider {
    pub fn new(fixed_time: NaiveDateTime) -> Self {
        Self { fixed_time }
    }
}

impl TimeProvider for MockTimeProvider {
    fn now(&self) -> NaiveDateTime {
        self.fixed_time
    }
}
