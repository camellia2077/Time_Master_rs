use anyhow::Result;
use colored::Colorize;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::time::{Duration, Instant};

mod config;
mod log_generator;

use config::{Config, JsonConfigData};
use log_generator::LogGenerator;

fn is_leap(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn get_days_in_month(year: i32, month: u32) -> u32 {
    match month {
        2 => if is_leap(year) { 29 } else { 28 },
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

fn main() -> Result<()> {
    let config = Config::from_args()?;

    // Phase 1: Configuration & Setup
    let json_config = JsonConfigData::from_file("activities_config.json")?;

    // Phase 2: Execution & Output
    let total_start_time = Instant::now();
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

    let mut text_generation_duration = Duration::new(0, 0);
    let mut io_duration = Duration::new(0, 0);

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

            // 1. Time the text generation
            let gen_start = Instant::now();
            let days_in_month = get_days_in_month(year, month);
            let month_log = generator.generate_for_month(month, days_in_month);
            text_generation_duration += gen_start.elapsed();

            // 2. Time the file I/O (creation and writing)
            let io_start = Instant::now();
            let mut out_file = File::create(&full_path)
                .map_err(|e| anyhow::anyhow!("Could not open file '{}' for writing: {}", full_path.display(), e))?;
            out_file.write_all(month_log.as_bytes())?;
            io_duration += io_start.elapsed();

            files_generated += 1;
        }
    }

    // Phase 3: Reporting
    let total_duration = total_start_time.elapsed();

    println!(
        "\n{}",
        format!(
            "Data generation complete. {} monthly log files created for years {}-{}.",
            files_generated, config.start_year, config.end_year
        )
        .green()
    );

    println!("---------------------------");
    println!(
        "total time:    {:.2} s ({}ms)",
        total_duration.as_secs_f64(),
        total_duration.as_millis()
    );
    println!(
        "text generate: {:.2} s ({}ms)",
        text_generation_duration.as_secs_f64(),
        text_generation_duration.as_millis()
    );
    println!(
        "io:            {:.2} s ({}ms)",
        io_duration.as_secs_f64(),
        io_duration.as_millis()
    );
    println!("---------------------------");

    Ok(())
}