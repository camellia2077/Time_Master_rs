use anyhow::Result;
use colored::Colorize;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::time::Instant;

mod config;
mod log_generator;
mod utils;

use config::{Config, JsonConfigData};
use log_generator::LogGenerator;

fn main() -> Result<()> {
    let config = Config::from_args()?;

    // Phase 1: Configuration & Setup
    let json_config = JsonConfigData::from_file("activities_config.json")?;

    // Phase 2: Execution & Output
    let start_time = Instant::now();
    println!(
        "Generating data for years {} to {}...",
        config.start_year, config.end_year
    );

    let mut generator = LogGenerator::new(
        config.items_per_day,
        &json_config.activities,
        json_config.remarks.as_ref(),
    );

    let mut files_generated = 0;
    let master_dir = "Date";

    // Create the master "Date" directory
    fs::create_dir_all(master_dir)?;
    println!("Created master directory: '{}'", master_dir);

    for year in config.start_year..=config.end_year {
        let year_dir = Path::new(master_dir).join(year.to_string());
        fs::create_dir_all(&year_dir)?;
        println!("Created directory: '{}'", year_dir.display());

        for month in 1..=12 {
            let filename = format!("{}_{:02}.txt", year, month);
            let full_path = year_dir.join(filename);

            let mut out_file = File::create(&full_path)
                .map_err(|e| anyhow::anyhow!("Could not open file '{}' for writing: {}", full_path.display(), e))?;

            let days_in_month = utils::get_days_in_month(year, month);
            let month_log = generator.generate_for_month(month, days_in_month);
            out_file.write_all(month_log.as_bytes())?;
            files_generated += 1;
        }
    }

    // Phase 3: Reporting
    let duration = start_time.elapsed();
    println!(
        "\n{}",
        format!(
            "Data generation complete. {} monthly log files created for years {}-{}.",
            files_generated, config.start_year, config.end_year
        )
        .green()
    );
    println!(
        "Total generation time: {:.3} s ({} ms).",
        duration.as_secs_f64(),
        duration.as_millis()
    );

    Ok(())
}