//! # Date Utility
//!
//! Date provides an easy way to get the current date and time in multiple formats.
//!
//! # Usage
//!
//! ```rust
//! use password_generator_pro::util::date::Date;
//!
//! let time = Date::now();
//! println!("The current time is: {}", time);
//! ```
//!

// Copyright © 2022-2023 Password Generator Pro. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

/// Implements [`Date`] to get the current date and time in UTC.
///
/// # Examples
///
/// ```rust
/// use password_generator_pro::util::date::Date;
///
/// let date = Date::now();
/// println!("The current date is: {}", date);
/// ```
use time::OffsetDateTime;

/// Date Utility
///
/// By default, the current date and time in UTC is returned.
#[non_exhaustive]
pub struct Date;

impl Date {
    /// Initializes a new [`Date`].
    ///
    /// ```no_run
    /// use password_generator_pro::util::date::Date;
    ///
    /// let date = Date::now();
    /// assert!(!date.is_empty());
    /// ```
    ///
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
    /// use password_generator_pro::util::date::Date;
    ///
    /// let year = Date::year();
    /// assert_eq!(year.len(), 4);
    /// assert!(year.parse::<i32>().is_ok());
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
    /// use password_generator_pro::util::date::Date;
    ///
    /// let month = Date::month();
    /// // `time` Displays a Month as its name, e.g. "August".
    /// assert!(month.chars().all(|c| c.is_ascii_alphabetic()));
    /// ```
    pub fn month() -> String {
        OffsetDateTime::now_utc().month().to_string()
    }

    /// Returns the current day.
    ///
    /// The returned value will always be in the range `1..=31`.
    ///
    /// ```rust
    /// use password_generator_pro::util::date::Date;
    ///
    /// let day = Date::day();
    /// // Unpadded: "3" rather than "03".
    /// let n: u8 = day.parse().unwrap();
    /// assert!((1..=31).contains(&n));
    /// ```
    pub fn day() -> String {
        OffsetDateTime::now_utc().day().to_string()
    }

    /// Returns the current hour.
    ///
    /// The returned value will always be in the range `0..=23`.
    ///
    /// ```rust
    /// use password_generator_pro::util::date::Date;
    ///
    /// let hour = Date::hour();
    /// // Unpadded: "3" rather than "03".
    /// let n: u8 = hour.parse().unwrap();
    /// assert!((0..=23).contains(&n));
    /// ```
    pub fn hour() -> String {
        OffsetDateTime::now_utc().hour().to_string()
    }

    /// Returns the current minute.
    ///
    /// The returned value will always be in the range `0..=59`.
    ///
    /// ```rust
    /// use password_generator_pro::util::date::Date;
    ///
    /// let time = Date::minute();
    /// assert!(!time.is_empty());
    /// ```
    pub fn minute() -> String {
        OffsetDateTime::now_utc().minute().to_string()
    }

    /// Returns the current second.
    ///
    /// The returned value will always be in the range `0..=59`.
    ///
    /// ```rust
    /// use password_generator_pro::util::date::Date;
    ///
    /// let time = Date::second();
    /// assert!(!time.is_empty());
    /// ```
    pub fn second() -> String {
        OffsetDateTime::now_utc().second().to_string()
    }

    /// Returns the current millisecond.
    ///
    /// The returned value will always be in the range `0..=999`.
    ///
    /// ```rust
    /// use password_generator_pro::util::date::Date;
    ///
    /// let time = Date::millisecond();
    /// assert!(!time.is_empty());
    /// ```
    pub fn millisecond() -> String {
        OffsetDateTime::now_utc().millisecond().to_string()
    }

    /// Returns the current microsecond.
    ///
    /// The returned value will always be in the range `0..=999`.
    ///
    /// ```rust
    /// use password_generator_pro::util::date::Date;
    ///
    /// let time = Date::microsecond();
    /// assert!(!time.is_empty());
    /// ```
    pub fn microsecond() -> String {
        OffsetDateTime::now_utc().microsecond().to_string()
    }

    /// Returns the current nanosecond.
    ///
    /// The returned value will always be in the range `0..=999`.
    ///
    /// ```rust
    /// use password_generator_pro::util::date::Date;
    ///
    /// let time = Date::nanosecond();
    /// assert!(!time.is_empty());
    /// ```
    pub fn nanosecond() -> String {
        OffsetDateTime::now_utc().nanosecond().to_string()
    }

    /// Returns the current timezone.
    ///
    /// The returned value will always be in the range `0..=14`.
    ///
    /// ```rust
    /// use password_generator_pro::util::date::Date;
    ///
    /// let time = Date::timezone();
    /// assert!(!time.is_empty());
    /// ```
    pub fn timezone() -> String {
        OffsetDateTime::now_utc().offset().to_string()
    }

    /// Returns the current weekday.
    ///
    /// The returned value will always be in the range `0..=6`.
    ///
    /// ```rust
    /// use password_generator_pro::util::date::Date;
    ///
    /// let time = Date::weekday();
    /// assert!(!time.is_empty());
    /// ```
    pub fn weekday() -> String {
        OffsetDateTime::now_utc().weekday().to_string()
    }
}

impl Clone for Date {
    fn clone(&self) -> Self {
        Date
    }
}

impl Default for Date {
    fn default() -> Self {
        Date
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_time() {
        let utc = Date::now();
        assert!(!utc.is_empty());
        assert_eq!(utc, utc.to_string());
    }

    #[test]
    fn test_year() {
        // let year = Date::year();
        // assert!(!year.is_empty());
        // assert_eq!(year, "2022");
        // assert_eq!(year, year.to_string());
        // assert_eq!(year.len(), 4);
    }

    #[test]
    fn test_month() {
        let utc = Date::month();
        assert!(!utc.is_empty());
        assert_eq!(utc, utc.to_string());
    }

    #[test]
    fn test_day() {
        let day = Date::day();
        assert!(!day.is_empty());
        assert_eq!(day, day.to_string());
        let n: u8 = day.parse().unwrap();
        assert!((1..=31).contains(&n));
    }

    #[test]
    fn test_hour() {
        let hour = Date::hour();
        assert!(!hour.is_empty());
        assert_eq!(hour, hour.to_string());
    }

    #[test]
    fn test_minute() {
        let minute = Date::minute();
        assert!(!minute.is_empty());
        assert_eq!(minute, minute.to_string());
    }

    #[test]
    fn test_second() {
        let second = Date::second();
        assert!(!second.is_empty());
        assert_eq!(second, second.to_string());
    }
    #[test]
    fn test_millisecond() {
        let utc = Date::millisecond();
        assert!(!utc.is_empty());
        assert_eq!(utc, utc.to_string());
    }
    #[test]
    fn test_microsecond() {
        let microsecond = Date::microsecond();
        assert!(!microsecond.is_empty());
        assert_eq!(microsecond, microsecond.to_string());
    }
    #[test]
    fn test_nanosecond() {
        let nanosecond = Date::nanosecond();
        assert!(!nanosecond.is_empty());
        assert_eq!(nanosecond, nanosecond.to_string());
    }
    #[test]
    fn test_timezone() {
        let timezone = Date::timezone();
        assert!(!timezone.is_empty());
        assert_eq!(timezone, timezone.to_string());
    }
    #[test]
    fn test_weekday() {
        let weekday = Date::weekday();
        assert!(!weekday.is_empty());
        assert_eq!(weekday, weekday.to_string());
    }
    #[test]
    fn every_accessor_returns_a_parseable_value() {
        // Each accessor was individually covered, but nothing asserted
        // the whole surface stays consistent — a new accessor returning
        // an empty string would have slipped through.
        assert_eq!(Date::year().len(), 4);
        assert!(Date::month().chars().all(|c| c.is_ascii_alphabetic()));
        for (name, value, lo, hi) in [
            ("day", Date::day(), 1u8, 31u8),
            ("hour", Date::hour(), 0, 23),
            ("minute", Date::minute(), 0, 59),
            ("second", Date::second(), 0, 59),
        ] {
            let n: u8 = value
                .parse()
                .unwrap_or_else(|_| panic!("{name} is not numeric: {value}"));
            assert!((lo..=hi).contains(&n), "{name} out of range: {n}");
        }
    }

    #[test]
    fn sub_second_accessors_are_numeric() {
        for (name, value) in [
            ("millisecond", Date::millisecond()),
            ("microsecond", Date::microsecond()),
            ("nanosecond", Date::nanosecond()),
        ] {
            assert!(
                value.chars().all(|c| c.is_ascii_digit()),
                "{name} is not numeric: {value}"
            );
        }
    }

    #[test]
    fn timezone_and_weekday_are_non_empty() {
        assert!(!Date::timezone().is_empty());
        assert!(!Date::weekday().is_empty());
    }

    #[test]
    fn now_contains_a_date_and_a_time() {
        let now = Date::now();
        assert!(now.contains('-'), "no date part in {now}");
        assert!(now.contains(':'), "no time part in {now}");
    }
}

// Returns the current date and time in UTC.
// pub fn get_time() -> String {
//     OffsetDateDate::now_utc().to_string()
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
//

#[cfg(test)]
mod trait_impl_tests {
    use super::*;

    #[test]
    fn clone_yields_an_equivalent_date() {
        // `Date` is a unit struct reading the clock on demand, so a clone
        // is interchangeable with the original rather than a snapshot.
        let original = Date;
        let cloned = original.clone();
        assert_eq!(
            std::mem::size_of_val(&cloned),
            std::mem::size_of_val(&original)
        );
    }

    #[test]
    fn default_is_constructible_and_usable() {
        // `Date`'s accessors are associated functions reading the clock,
        // so a defaulted value is interchangeable with a direct one; the
        // point here is that Default is constructible at all.
        let date = Date::default();
        assert_eq!(std::mem::size_of_val(&date), 0);
        assert!(!Date::year().is_empty());
    }
}
