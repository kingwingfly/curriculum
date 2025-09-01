use bon::bon;
use chrono::{NaiveDateTime, TimeDelta};
use getset::Getters;

#[derive(Debug, Getters, PartialEq, Eq, Clone, Copy, Hash)]
pub struct CoursePeriod {
    /// Local TZ
    #[getset(get = "pub")]
    start: NaiveDateTime,
    /// Local TZ
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

impl CoursePeriod {
    pub fn contains(&self, data_time: NaiveDateTime) -> bool {
        self.start <= data_time && data_time <= self.end
    }
}
