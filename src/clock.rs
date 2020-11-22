//! A clock module
use chrono::prelude::*;
use std::fmt;

/// A date and time status module
///
/// Displays as `%Y-%m-%d %H:%M:%S`
pub struct Clock;

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let datetime = Local::now();
        write!(f, "{}", datetime.format("%Y-%m-%d %H:%M:%S"))
    }
}
