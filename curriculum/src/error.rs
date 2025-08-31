use thiserror::Error;

#[derive(Debug, Error)]
pub enum CourseError {}

pub type Result<T> = core::result::Result<T, CourseError>;
