#![feature(decl_macro)]

pub mod course;
pub mod curriculum;
pub mod macros;
pub mod period;
pub mod query;
pub mod time_provider;
pub mod tutor;

pub use course::Course;
pub use macros::course;
pub use period::Period;
pub use query::Query;
pub use time_provider::{DefaultTimeProvider, MockTimeProvider};
pub use tutor::Tutor;

pub use chrono::{Days, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta};
