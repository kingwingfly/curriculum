use chrono::{Local, NaiveDateTime};

pub trait TimeProvider {
    fn now(&self) -> NaiveDateTime {
        Local::now().naive_local()
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultTimeProvider;

impl TimeProvider for DefaultTimeProvider {}

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
