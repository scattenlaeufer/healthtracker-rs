use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs::{read_to_string, File};
use std::io::prelude::*;
use std::io::BufWriter;

pub const DATE_FORMAT: &str = "%Y-%m-%d";
const DATA_FILE_NAME: &str = "data.ron";
const BIKING_DISTANCE: f32 = 10.0;

#[derive(Debug)]
pub enum HealthTrackerError {
    Dummy(String),
    ChronoParse(String),
    XDGBaseDirectories(String),
    IOError(String),
    Ron(String),
}

impl std::error::Error for HealthTrackerError {}

impl fmt::Display for HealthTrackerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Dummy(e) => write!(f, "Some dummy error: {}", e),
            Self::ChronoParse(e) => write!(f, "Chrono Parse Error: {}", e),
            Self::XDGBaseDirectories(e) => write!(f, "XDG BaseDirectories Error: {}", e),
            Self::IOError(e) => write!(f, "IO Error: {}", e),
            Self::Ron(e) => write!(f, "RON Error: {}", e),
        }
    }
}

impl From<chrono::ParseError> for HealthTrackerError {
    fn from(error: chrono::ParseError) -> Self {
        Self::ChronoParse(error.to_string())
    }
}

impl From<xdg::BaseDirectoriesError> for HealthTrackerError {
    fn from(error: xdg::BaseDirectoriesError) -> Self {
        Self::XDGBaseDirectories(error.to_string())
    }
}

impl From<std::io::Error> for HealthTrackerError {
    fn from(error: std::io::Error) -> Self {
        Self::IOError(error.to_string())
    }
}

impl From<ron::Error> for HealthTrackerError {
    fn from(error: ron::Error) -> Self {
        Self::Ron(error.to_string())
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Day {
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
    fn new(weight: Option<f32>, workout: bool, training: bool, biking: Option<f32>) -> Self {
        Self {
            weight,
            workout,
            training,
            biking,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct History {
    map: HashMap<NaiveDate, Day>,
}

impl History {
    fn load() -> Result<Self, HealthTrackerError> {
        let xdg_basedir = xdg::BaseDirectories::with_prefix(clap::crate_name!())?;
        let history = match xdg_basedir.find_data_file(DATA_FILE_NAME) {
            Some(p) => ron::from_str::<History>(&read_to_string(p)?)?,
            None => History {
                map: HashMap::new(),
            },
        };
        Ok(history)
    }
}

impl History {
    fn log_weight(&mut self, date: NaiveDate, weight: f32) {
        let day = if let Some(day) = self.map.get(&date) {
            Day::new(Some(weight), day.workout, day.training, day.biking)
        } else {
            Day::new(Some(weight), false, false, None)
        };
        self.map.insert(date, day);
    }

    fn log_sport(&mut self, date: NaiveDate, workout: bool, training: bool, biking: Option<f32>) {
        let day = if let Some(day) = self.map.get(&date) {
            Day::new(
                day.weight,
                day.workout || workout,
                day.training || training,
                match biking {
                    Some(d) => Some(d),
                    None => day.biking,
                },
            )
        } else {
            Day::new(None, workout, training, biking)
        };
        self.map.insert(date, day);
    }
}

impl History {
    fn save(&self) -> Result<(), HealthTrackerError> {
        let xdg_basedir = xdg::BaseDirectories::with_prefix(clap::crate_name!())?;
        let path = xdg_basedir.place_data_file(DATA_FILE_NAME)?;
        let file = File::create(&path)?;
        let mut writer = BufWriter::new(&file);
        write!(&mut writer, "{}", ron::ser::to_string(&self)?)?;
        Ok(())
    }

    fn get_sport_streak(&self, date: NaiveDate) -> u32 {
        let day = match self.map.get(&date) {
            Some(d) => d,
            None => return 0,
        };

        if day.workout || day.training || day.biking.unwrap_or(0.0) >= BIKING_DISTANCE {
            1 + self.get_sport_streak(date.pred())
        } else {
            0
        }
    }
}

fn get_date(date_str: Option<String>) -> Result<NaiveDate, HealthTrackerError> {
    match date_str {
        Some(s) => Ok(NaiveDate::parse_from_str(&s, DATE_FORMAT)?),
        None => Ok(Local::today().naive_local()),
    }
}

pub fn log_weight(weight: f32, date_str: Option<String>) -> Result<(), HealthTrackerError> {
    let mut history = History::load()?;
    let date = get_date(date_str)?;
    history.log_weight(date, weight);
    history.save()?;

    Ok(())
}

pub fn log_sport(
    workout: bool,
    training: bool,
    biking: Option<f32>,
    date_str: Option<String>,
) -> Result<(), HealthTrackerError> {
    let mut history = History::load()?;
    let date = get_date(date_str)?;
    history.log_sport(date, workout, training, biking);
    history.save()?;

    Ok(())
}

pub fn analyze() -> Result<(), HealthTrackerError> {
    let history = History::load()?;
    println!("{:#?}", history);

    let sport_streak = history.get_sport_streak(get_date(None)?);

    println!("Current sport streak: {}", sport_streak);

    Ok(())
}
