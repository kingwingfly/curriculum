use bon::bon;
use chrono::{NaiveDateTime, TimeDelta};
use getset::Getters;

pub type Holiday = Period;

#[derive(Debug, Getters, PartialEq, Eq, Clone, Hash)]
pub struct Period {
    #[getset(get = "pub")]
    name: Option<String>,
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
impl Period {
    #[builder]
    pub fn new(name: Option<&str>, start: NaiveDateTime, duration: TimeDelta) -> Self {
        Self {
            name: name.map(|s| s.to_owned()),
            start,
            end: start + duration,
            duration,
        }
    }

    pub fn is_overlap(&self, other: &Self) -> bool {
        !(self.end <= other.start || self.start >= other.end)
    }
}
