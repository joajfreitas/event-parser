use chrono::{NaiveTime, Utc};
use regex::*;
use std::error::Error;
use std::fmt;

// use crate::date_parse::*;
use crate::recognizable::Recognizable;

extern crate regex;

#[derive(Debug, PartialEq)]
pub enum TimeParseError {
    TimeUnknown,
    TimeBad,
}

impl fmt::Display for TimeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TimeParseError::TimeUnknown => write!(f, "Error: Time unknown"),
            TimeParseError::TimeBad => write!(f, "Error: Time bad format"),
        }
    }
}

impl Error for TimeParseError {
    fn description(&self) -> &str {
        "Time unknown"
    }
}

// Time Parser
pub struct TimeParser {}

impl TimeParser {
    pub fn parse(&self, text: &str) -> Result<NaiveTime, TimeParseError> {
        self.parse_relative(text, Some(&Utc::now().time()))
    }

    pub fn parse_relative(
        &self,
        text: &str,
        now: Option<&NaiveTime>,
    ) -> Result<NaiveTime, TimeParseError> {
        unimplemented!()
        // TimeExpr.recognize(text)

        // match TimeExpr
        // create NaiveTime based on DateExpr and now
    }
}

#[derive(Debug, PartialEq)]
enum TimeExpr {
    Now,
    Absolute(NaiveTime),
    // InDay(Box<DateExpr>),
    InNHours(usize),
    InNMins(usize),
}

// https://github.com/wanasit/chrono/blob/master/src/parsers/en/ENTimeExpressionParser.js
impl Recognizable for TimeExpr {
    type Error = TimeParseError;

    fn recognize(text: &str) -> Result<TimeExpr, Self::Error> {
        match try_absolute_time(text) {
            Some(expr) => Ok(expr),
            None => Err(TimeParseError::TimeUnknown),
        }
    }

    fn describe() -> &'static str {
        "time of day"
    }
}

fn try_absolute_time(text: &str) -> Option<TimeExpr> {
    // colon, "am", "pm", "o'clock", ...?

    // 10:30am/pm AM/PM a/p A/P
    let re = Regex::new(r"(?i)\d{1,2}:\d{2}[ap]m?").unwrap();

    if let Some(time) = re.find(text) {
        let mut time_str = time.as_str().to_lowercase();

        if !time_str.ends_with("m") {
            time_str.push('m');
        }
        if let Ok(nt) = NaiveTime::parse_from_str(&time_str, "%l:%M%P") {
            return Some(TimeExpr::Absolute(nt));
        };
    }

    // 10:30
    let re = Regex::new(r"\d{1,2}:\d{2}").unwrap();

    if let Some(time) = re.find(text) {
        let time_str = time.as_str().to_lowercase();

        if let Ok(nt) = NaiveTime::parse_from_str(&time_str, "%k:%M") {
            return Some(TimeExpr::Absolute(nt));
        }
    }

    // 10pm/am a/p
    let re = Regex::new(r"(?i)\d{2}[ap]m?").unwrap();

    if let Some(time) = re.find(text) {
        let mut time_str = time.as_str().to_lowercase();

        let (hour, pm) = time_str.split_at(2);
        let mut hour: u32 = hour.parse().unwrap();
        if pm.contains("p") {
            hour += 12;
        }
        return Some(TimeExpr::Absolute(NaiveTime::from_hms(hour, 0, 0)));
    }

    // 2pm
    let re = Regex::new(r"(?i)\d{1}[ap]m?").unwrap();

    if let Some(time) = re.find(text) {
        let mut time_str = time.as_str().to_lowercase();

        let (hour, pm) = time_str.split_at(1);
        let mut hour: u32 = hour.parse().unwrap();
        if pm.contains("p") {
            hour += 12;
        }
        return Some(TimeExpr::Absolute(NaiveTime::from_hms(hour, 0, 0)));
    }

    // 10
    let re = Regex::new(r"\d{1,2}").unwrap();

    if let Some(time) = re.find(text) {
        let mut hour: u32 = time.as_str().parse().unwrap();
        if hour < 8 && !hour > 12 {
            hour += 12;
        }
        return Some(TimeExpr::Absolute(NaiveTime::from_hms(hour, 0, 0)));
    }

    None
}

fn try_casual_time(text: &str) -> Option<TimeExpr> {
    // "morning", "evening", "midnight", "mid{-}?day", ...?
    None
}

fn try_relative_time(text: &str) -> Option<TimeExpr> {
    // "in_hours/minutes",
    None
}

// Tests
#[cfg(test)]
mod time_expr_tests {
    use super::{Recognizable, TimeExpr};
    use chrono::NaiveTime;

    #[test]
    fn simple_hour_tests() {
        assert_recognize_time("12", 12, 0);
        assert_recognize_time("2", 14, 0);
        assert_recognize_time("10", 10, 0);
        assert_recognize_time("5", 17, 0);
    }

    #[test]
    fn am_pm_hour_tests() {
        assert_recognize_time("10am", 10, 0);
        assert_recognize_time("10pm", 22, 0);
        assert_recognize_time("2p", 14, 0);
    }

    #[test]
    fn simple_minute_tests() {
        assert_recognize_time("12:30", 12, 30);
        assert_recognize_time("2:30", 2, 30);
    }

    #[test]
    fn am_pm_minute_tests() {
        assert_recognize_time("10:30am", 10, 30);
        assert_recognize_time("2:30pm", 14, 30);
        assert_recognize_time("10:30AM", 10, 30);
        assert_recognize_time("2:30PM", 14, 30);
        assert_recognize_time("10:30a", 10, 30);
        assert_recognize_time("2:30p", 14, 30);
        // assert_recognize_time("10pm", 22, 30);
    }

    fn assert_recognize_time(text: &str, expected_h: u32, expected_m: u32) {
        assert_eq!(
            TimeExpr::recognize(text),
            Ok(TimeExpr::Absolute(NaiveTime::from_hms(
                expected_h, expected_m, 0
            )))
        )
    }
}
