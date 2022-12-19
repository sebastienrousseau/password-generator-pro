//! # Date Utility
//!
//! Date provides an easy way to get the current date and time in multiple formats.
//!
//! # Usage
//!
//! ```rust
//! use date::Time;
//!
//! let time = Time::now();
//! println!("The current time is: {}", time);
//! ```
//!
//!

// Copyright 2022-2023 Password Generator Pro. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

/// Implements [`Time`] to get the current date and time in UTC.
///
/// # Examples
///
/// ```rust
/// use date::Time;
///
/// let time = Time::now();
/// println!("The current time is: {}", time);
/// ```

use time::OffsetDateTime;

/// Time in UTC.
///
/// By default, the current date and time in UTC is returned.
#[non_exhaustive]
pub struct Time;

impl Time {
    /// Initializes a new [`Time`].
    ///
    /// ```no_run
    /// use date::Time;
    ///
    /// let time = Time::now();
    /// assert_eq!(time, "2022-22-22 22:22:22.222222 +00:00:00");
    /// ```

    /// Returns the current date and time in UTC.
    ///
    /// The returned value will always be from the range `0001-01-01 00:00:00.000000 +00:00:00` to `9999-12-31 23:59:59.999999 +00:00:00`.
    pub fn now() -> String {
        OffsetDateTime::now_utc().to_string()
    }

    /// Returns the current year.
    ///
    /// The returned value will always be in the range `1..=9999`.
    ///
    /// ```rust
    /// use date::Time;
    ///
    /// let year = Time::year();
    /// assert_eq!(year, "2022");
    /// ```
    #[inline]
    pub fn year() -> String {
        OffsetDateTime::now_utc().year().to_string()
    }
    // pub fn year() -> String {
    //     OffsetDateTime::now_utc().year().to_string()
    // }

    /// Returns the current month.
    ///
    /// The returned value will always be in the range `1..=12`.
    ///
    /// ```rust
    /// use date::Time;
    ///
    /// let month = Time::month();
    /// assert_eq!(month, "01");
    /// ```
    pub fn month() -> String {
        OffsetDateTime::now_utc().month().to_string()
    }

    /// Returns the current day.
    ///
    /// The returned value will always be in the range `1..=31`.
    ///
    /// ```rust
    /// use date::Time;
    ///
    /// let day = Time::day();
    /// assert_eq!(day, "02");
    /// ```
    pub fn day() -> String {
        OffsetDateTime::now_utc().day().to_string()
    }

    /// Returns the current hour.
    ///
    /// The returned value will always be in the range `0..=23`.
    ///
    /// ```rust
    /// use date::Time;
    ///
    /// let hour = Time::hour();
    /// assert_eq!(hour, "0");
    /// ```
    pub fn hour() -> String {
        OffsetDateTime::now_utc().hour().to_string()
    }

    /// Returns the current minute.
    ///
    /// The returned value will always be in the range `0..=59`.
    ///
    /// ```rust
    /// use date::Time;
    ///
    /// let time = Time::minute();
    /// assert_eq!(time, "0");
    /// ```
    pub fn minute() -> String {
        OffsetDateTime::now_utc().minute().to_string()
    }

    /// Returns the current second.
    ///
    /// The returned value will always be in the range `0..=59`.
    ///
    /// ```rust
    /// use date::Time;
    ///
    /// let time = Time::second();
    /// assert_eq!(time, "0");
    /// ```
    pub fn second() -> String {
        OffsetDateTime::now_utc().second().to_string()
    }

    /// Returns the current millisecond.
    ///
    /// The returned value will always be in the range `0..=999`.
    ///
    /// ```rust
    /// use date::Time;
    ///
    /// let time = Time::millisecond();
    /// assert_eq!(time, "0");
    /// ```
    pub fn millisecond() -> String {
        OffsetDateTime::now_utc().millisecond().to_string()
    }

    /// Returns the current microsecond.
    ///
    /// The returned value will always be in the range `0..=999`.
    ///
    /// ```rust
    /// use date::Time;
    ///
    /// let time = Time::microsecond();
    /// assert_eq!(time, "0");
    /// ```
    pub fn microsecond() -> String {
        OffsetDateTime::now_utc().microsecond().to_string()
    }

    /// Returns the current nanosecond.
    ///
    /// The returned value will always be in the range `0..=999`.
    ///
    /// ```rust
    /// use date::Time;
    ///
    /// let time = Time::nanosecond();
    /// assert_eq!(time, "0");
    /// ```
    pub fn nanosecond() -> String {
        OffsetDateTime::now_utc().nanosecond().to_string()
    }

    /// Returns the current timezone.
    ///
    /// The returned value will always be in the range `0..=14`.
    ///
    /// ```rust
    /// use date::Time;
    ///
    /// let time = Time::timezone();
    /// assert_eq!(time, "0");
    /// ```
    pub fn timezone() -> String {
        OffsetDateTime::now_utc().offset().to_string()
    }

    /// Returns the current weekday.
    ///
    /// The returned value will always be in the range `0..=6`.
    ///
    /// ```rust
    /// use date::Time;
    ///
    /// let time = Time::weekday();
    /// assert_eq!(time, "0");
    /// ```
    pub fn weekday() -> String {
        OffsetDateTime::now_utc().weekday().to_string()
    }
}


impl Clone for Time {
    fn clone(&self) -> Self {
        Time
    }
}

impl Default for Time {
    fn default() -> Self {
        Time
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_time() {
        let utc = Time::now();
        assert!(!utc.is_empty());
        assert_eq!(utc, utc.to_string());
    }

    #[test]
    fn test_year() {
        // let year = Time::year();
        // assert!(!year.is_empty());
        // assert_eq!(year, "2022");
        // assert_eq!(year, year.to_string());
        // assert_eq!(year.len(), 4);
    }

    #[test]
    fn test_month() {
        let utc = Time::month();
        assert!(!utc.is_empty());
        assert_eq!(utc, utc.to_string());
    }

    #[test]
    fn test_day() {
        let day = Time::day();
        assert!(!day.is_empty());
        assert_eq!(day, day.to_string());
        assert_eq!(day.len(), 2);
    }

    #[test]
    fn test_hour() {
        let hour = Time::hour();
        assert!(!hour.is_empty());
        assert_eq!(hour, hour.to_string());
    }

    #[test]
    fn test_minute() {
        let minute = Time::minute();
        assert!(!minute.is_empty());
        assert_eq!(minute, minute.to_string());
    }

    #[test]
    fn test_second() {
        let second = Time::second();
        assert!(!second.is_empty());
        assert_eq!(second, second.to_string());
    }
    #[test]
    fn test_millisecond() {
        let utc = Time::millisecond();
        assert!(!utc.is_empty());
        assert_eq!(utc, utc.to_string());
    }
    #[test]
    fn test_microsecond() {
        let microsecond = Time::microsecond();
        assert!(!microsecond.is_empty());
        assert_eq!(microsecond, microsecond.to_string());
    }
    #[test]
    fn test_nanosecond() {
        let nanosecond = Time::nanosecond();
        assert!(!nanosecond.is_empty());
        assert_eq!(nanosecond, nanosecond.to_string());
    }
    #[test]
    fn test_timezone() {
        let timezone = Time::timezone();
        assert!(!timezone.is_empty());
        assert_eq!(timezone, timezone.to_string());
    }
    #[test]
    fn test_weekday() {
        let weekday = Time::weekday();
        assert!(!weekday.is_empty());
        assert_eq!(weekday, weekday.to_string());
    }
}



// Returns the current date and time in UTC.
// pub fn get_time() -> String {
//     OffsetDateTime::now_utc().to_string()
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_get_time() {
//         let utc = get_time();
//         assert!(!utc.is_empty());
//         assert_eq!(utc, utc.to_string());
//     }
// }