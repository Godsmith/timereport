use crate::timedelta::{deserialize_timedelta, serialize_timedelta};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

use crate::day::Day;
use chrono::{NaiveDate, TimeDelta};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub project_names: Vec<String>,
    days: Vec<Day>,
    undone: Vec<Day>,
    #[serde(deserialize_with = "deserialize_timedelta")]
    #[serde(serialize_with = "serialize_timedelta")]
    pub working_time_per_day: TimeDelta,
}

impl Config {
    fn new(days: Vec<Day>) -> Self {
        Self {
            project_names: Vec::new(),
            days,
            undone: Vec::new(),
            // TODO: make customizable
            working_time_per_day: TimeDelta::new(27900, 0).expect("hardcoded seconds"),
        }
    }
    pub fn save(&self, path: &Path) {
        let json_string = serde_json::to_string_pretty(&self).unwrap();
        match fs::write(path, json_string) {
            Ok(_) => {}
            Err(_) => eprintln!("Error writing to file {}", path.to_string_lossy()),
        }
    }
    pub fn add_project(&mut self, name: String) -> () {
        self.project_names.push(name);
    }

    pub fn add_day(&mut self, day: Day) -> () {
        self.days.push(day);
        self.undone.clear();
    }

    pub fn undo(&mut self) -> Result<NaiveDate, String> {
        let day = match self.days.pop() {
            Some(day) => day,
            None => return Err("Nothing to undo".to_string()),
        };
        let date = day.date;
        self.undone.push(day);
        Ok(date)
    }

    pub fn redo(&mut self) -> Result<NaiveDate, String> {
        let day = match self.undone.pop() {
            Some(day) => day,
            None => return Err("Nothing to redo".to_string()),
        };
        let date = day.date;
        self.days.push(day);
        Ok(date)
    }

    pub fn day_from_date(&self) -> HashMap<NaiveDate, Day> {
        let mut day_from_date: HashMap<NaiveDate, Day> = HashMap::new();
        for day in &self.days {
            match day_from_date.get(&day.date) {
                None => day_from_date.insert(day.date, day.clone()),
                Some(old_day) => day_from_date.insert(day.date, old_day.combine(day)),
            };
        }
        day_from_date
    }
}

pub fn load(path: &Path) -> Result<Config, String> {
    if fs::metadata(path).is_err() {
        create_empty_config_file(path);
    }
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(error) => {
            return Err(format!(
                "Error when trying to access {}: {}",
                path.to_string_lossy(),
                error.to_string()
            ))
        }
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect(&format!("Failed to read {}", path.to_string_lossy()));
    serde_json::from_str(&contents).expect(&format!("Failed to parse {}", path.to_string_lossy()))
}

fn create_empty_config_file(path: &Path) {
    fs::File::create(path).expect(&format!("Failed to create file {}", path.to_string_lossy()));
    let config = Config::new(Vec::new());
    config.save(path);
}
