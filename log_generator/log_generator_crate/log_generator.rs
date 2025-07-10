use crate::config::DailyRemarkConfig;
use rand::distributions::{Bernoulli, Uniform};
use rand::rngs::ThreadRng;
use rand::Rng;
use std::fmt::Write;

pub struct LogGenerator<'a> {
    items_per_day: u32,
    common_activities: &'a [String],
    remark_config: Option<&'a DailyRemarkConfig>,
    rng: ThreadRng,
}

impl<'a> LogGenerator<'a> {
    pub fn new(
        items_per_day: u32,
        activities: &'a [String],
        remark_config: Option<&'a DailyRemarkConfig>,
    ) -> Self {
        Self {
            items_per_day,
            common_activities: activities,
            remark_config,
            rng: rand::thread_rng(),
        }
    }

    pub fn generate_for_month(&mut self, month: u32, days_in_month: u32) -> String {
        let mut log_content = String::new();
        let minute_dist = Uniform::from(0..=59);
        let activity_dist = Uniform::from(0..self.common_activities.len());

        for day in 1..=days_in_month {
            if day > 1 {
                log_content.push('\n');
            }

            writeln!(&mut log_content, "{:02}{:02}", month, day).unwrap();

            if let Some(config) = self.remark_config {
                let remark_dist = Bernoulli::new(config.generation_chance).unwrap();
                if self.rng.sample(remark_dist) {
                    let remark_content_dist = Uniform::from(0..config.contents.len());
                    let content_idx = self.rng.sample(remark_content_dist);
                    writeln!(
                        &mut log_content,
                        "{}{}",
                        config.prefix, &config.contents[content_idx]
                    )
                    .unwrap();
                }
            }

            for i in 0..self.items_per_day {
                let (hour, minute, activity) = if i == 0 {
                    // FIX 1: Cast the integer literal '6' to a u32 to match the 'else' branch.
                    (6 as u32, self.rng.sample(minute_dist), "起床")
                } else {
                    let progress_ratio = if self.items_per_day > 1 {
                        i as f64 / (self.items_per_day - 1) as f64
                    } else {
                        1.0
                    };
                    let logical_hour = 6 + (progress_ratio * 19.0).round() as u32;
                    let display_hour = if logical_hour >= 24 { logical_hour - 24 } else { logical_hour };
                    let activity_idx = self.rng.sample(activity_dist);
                    (
                        display_hour,
                        self.rng.sample(minute_dist),
                        // FIX 2: Explicitly convert the &String to a &str slice.
                        &self.common_activities[activity_idx][..],
                    )
                };

                writeln!(
                    &mut log_content,
                    "{:02}{:02}{}",
                    hour, minute, activity
                )
                .unwrap();
            }
        }
        log_content
    }
}