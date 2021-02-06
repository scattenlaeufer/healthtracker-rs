use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs::{read_to_string, File};
use std::io::prelude::*;
use std::io::BufWriter;

pub const DATE_FORMAT: &str = "%Y-%m-%d";
const DATA_FILE_NAME: &str = "data.ron";

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
    fn save(&self) -> Result<(), HealthTrackerError> {
        let xdg_basedir = xdg::BaseDirectories::with_prefix(clap::crate_name!())?;
        let path = xdg_basedir.place_data_file(DATA_FILE_NAME)?;
        let file = File::create(&path)?;
        let mut writer = BufWriter::new(&file);
        write!(&mut writer, "{}", ron::ser::to_string(&self)?)?;
        Ok(())
    }
}

pub fn log_weight(weight: f32, date_str: Option<String>) -> Result<(), HealthTrackerError> {
    let history = History::load()?;
    println!("{:#?}", history);
    println!("{:?}", ron::ser::to_string(&history));
    history.save()?;

    let day = Day::new(date_str, Some(weight), false, false, None)?;
    println!("{:#?}", day);
    Ok(())
}
