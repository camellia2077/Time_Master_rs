use anyhow::{anyhow, Context, Result};
use clap::Parser;
use colored::Colorize;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version="2.2.0", about, long_about = None)]
#[command(override_usage = format!("{} <START_YEAR> <END_YEAR> <ITEMS_PER_DAY>", "rust_log_generator".bright_yellow()))]
pub struct Config {
    #[arg(help = "The starting year (e.g., 1990)")]
    pub start_year: i32,
    #[arg(help = "The ending year (inclusive)")]
    pub end_year: i32,
    #[arg(help = "Number of log items per day (positive integer)")]
    pub items_per_day: u32,
}

impl Config {
    pub fn from_args() -> Result<Self> {
        let config = Self::parse();
        if config.start_year <= 0 || config.end_year <= 0 || config.items_per_day == 0 {
            return Err(anyhow!("Years and <items_per_day> must be positive integers."));
        }
        if config.end_year < config.start_year {
            return Err(anyhow!("<end_year> cannot be earlier than <start_year>."));
        }
        Ok(config)
    }
}

#[derive(Deserialize, Debug)]
pub struct DailyRemarkConfig {
    pub prefix: String,
    pub contents: Vec<String>,
    #[serde(default = "default_generation_chance")]
    pub generation_chance: f64,
}

fn default_generation_chance() -> f64 {
    0.5
}

#[derive(Deserialize, Debug)]
pub struct JsonConfigData {
    #[serde(rename = "common_activities")]
    pub activities: Vec<String>,
    #[serde(rename = "daily_remarks")]
    pub remarks: Option<DailyRemarkConfig>,
}

impl JsonConfigData {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let file_content = fs::read_to_string(path)
            .with_context(|| format!("Could not open or read configuration file '{}'", path.display()))?;
        
        let mut data: Self = serde_json::from_str(&file_content)
            .with_context(|| format!("Failed to parse JSON from '{}'", path.display()))?;

        if data.activities.is_empty() {
            return Err(anyhow!("'common_activities' array in '{}' must not be empty.", path.display()));
        }

        println!(
            "{} Successfully loaded {} activities from '{}'.",
            "✔".green(),
            data.activities.len(),
            path.display()
        );

        if let Some(remarks_config) = &mut data.remarks {
            if remarks_config.contents.is_empty() {
                eprintln!(
                    "{} 'daily_remarks' object in '{}' has an empty 'contents' array. This feature will be disabled.",
                    "Warning:".yellow(), path.display()
                );
                data.remarks = None;
            } else if !(0.0..=1.0).contains(&remarks_config.generation_chance) {
                eprintln!(
                    "{} 'generation_chance' in '{}' must be between 0.0 and 1.0. Using default of 0.5.",
                    "Warning:".yellow(), path.display()
                );
                remarks_config.generation_chance = 0.5;
            } else {
                 println!(
                    "{} Successfully loaded {} daily remarks with a {}% generation chance.",
                    "✔".green(),
                    remarks_config.contents.len(),
                    remarks_config.generation_chance * 100.0
                );
            }
        }

        Ok(data)
    }
}