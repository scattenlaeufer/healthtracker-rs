use chrono::prelude::*;
use std::fmt;

pub const DATE_FORMAT: &str = "%Y-%m-%d";

#[derive(Debug)]
pub enum HealthTrackerError {
    ChronoParse(String),
}

impl std::error::Error for HealthTrackerError {}

impl fmt::Display for HealthTrackerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HealthTrackerError::ChronoParse(e) => write!(f, "Chrono Parse Error: {}", e),
        }
    }
}

impl From<chrono::ParseError> for HealthTrackerError {
    fn from(error: chrono::ParseError) -> Self {
        HealthTrackerError::ChronoParse(error.to_string())
    }
}

#[derive(Debug)]
struct Day {
    date: NaiveDate,
    /// weight of the day
    weight: Option<f32>,
    /// did I do my 7 minute workout?
    workout: bool,
    /// did I do a propper training session?
    training: bool,
    /// how much did I bike on that day?
    biking: Option<f32>,
}

impl Day {
    fn new(
        date_str: Option<String>,
        weight: Option<f32>,
        workout: bool,
        training: bool,
        biking: Option<f32>,
    ) -> Result<Self, HealthTrackerError> {
        let date = match date_str {
            Some(s) => NaiveDate::parse_from_str(&s, DATE_FORMAT)?,
            None => Local::today().naive_local(),
        };
        Ok(Self {
            date,
            weight,
            workout,
            training,
            biking,
        })
    }
}

pub fn log_weight(weight: f32, date_str: Option<String>) -> Result<(), HealthTrackerError> {
    let day = Day::new(date_str, Some(weight), false, false, None)?;
    println!("{:#?}", day);
    Ok(())
}
