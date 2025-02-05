use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::day::Day;

#[derive(Serialize, Deserialize)]
pub struct Config {
    days: Vec<Day>,
    undone: Vec<Day>,
}

impl Config {
    fn new(days: Vec<Day>) -> Self {
        Self {
            days,
            undone: Vec::new(),
        }
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

    pub fn day_from_date(self) -> HashMap<NaiveDate, Day> {
        let mut day_from_date: HashMap<NaiveDate, Day> = HashMap::new();
        for day in self.days {
            match day_from_date.get(&day.date) {
                None => day_from_date.insert(day.date, day),
                Some(old_day) => day_from_date.insert(day.date, old_day.combine(day)),
            };
        }
        day_from_date
    }
}

pub fn load_config(path: &Path) -> Config {
    if fs::metadata(path).is_err() {
        create_empty_config_file(path);
    }
    let mut file = File::open(path).expect("Failed to open days.json");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect(&format!("Failed to read {}", path.to_string_lossy()));
    serde_json::from_str(&contents).expect(&format!("Failed to parse {}", path.to_string_lossy()))
}

pub fn save_config(config: &Config, path: &Path) {
    let json_string = serde_json::to_string_pretty(&config).unwrap();
    match fs::write(path, json_string) {
        Ok(_) => {}
        Err(_) => eprintln!("Error writing to file {}", path.to_string_lossy()),
    }
}

fn create_empty_config_file(path: &Path) {
    fs::File::create(path).expect(&format!("Failed to create file {}", path.to_string_lossy()));
    let config = Config::new(Vec::new());
    save_config(&config, path);
}
