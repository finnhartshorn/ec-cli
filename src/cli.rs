use clap::{Parser, Subcommand};
use chrono::{Datelike, NaiveDate, Utc, Weekday};

/// Calculate the default quest year based on current date
///
/// Everybody Codes launches on the first Monday of November at 11pm UTC.
///
/// Returns current year if after 11pm UTC on first Monday of November,
/// otherwise returns previous year.
///
/// Examples:
/// - 2024-11-04 22:59 UTC -> 2023 (before cutoff)
/// - 2024-11-04 23:00 UTC -> 2024 (at cutoff)
/// - 2024-11-05 00:00 UTC -> 2024 (after cutoff)
fn default_year() -> String {
    let now = Utc::now();
    let current_year = now.year();

    // Find first Monday of November
    let nov_1 = NaiveDate::from_ymd_opt(current_year, 11, 1).unwrap();
    let mut first_monday = nov_1;

    // Find the first Monday
    while first_monday.weekday() != Weekday::Mon {
        first_monday = first_monday.succ_opt().unwrap();
    }

    // Create the cutoff datetime: first Monday of November at 23:00 UTC
    let cutoff = first_monday.and_hms_opt(23, 0, 0).unwrap();
    let cutoff_datetime = chrono::DateTime::<Utc>::from_naive_utc_and_offset(cutoff, Utc);

    // If we're past the cutoff, use current year; otherwise use previous year
    if now >= cutoff_datetime {
        current_year.to_string()
    } else {
        (current_year - 1).to_string()
    }
}

#[derive(Parser)]
#[command(name = "ec-cli")]
#[command(about = "CLI for Everybody Codes puzzles", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable debug logging
    #[arg(long, global = true)]
    pub debug: bool,

    /// Suppress non-error output
    #[arg(long, global = true)]
    pub quiet: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Download and decrypt puzzle inputs and descriptions
    Fetch {
        /// Quest year
        #[arg(short, long, default_value_t = default_year().parse().unwrap())]
        year: i32,

        /// Quest day (1-20)
        #[arg(short, long)]
        day: i32,

        /// Quest part (1-3)
        #[arg(short, long)]
        part: i32,

        /// Download description only (skip input)
        #[arg(long)]
        description_only: bool,

        /// Download input only (skip description)
        #[arg(long)]
        input_only: bool,
    },

    /// Display puzzle description in terminal
    Read {
        /// Quest year
        #[arg(short, long, default_value_t = default_year().parse().unwrap())]
        year: i32,

        /// Quest day (1-20)
        #[arg(short, long)]
        day: i32,

        /// Terminal width for text wrapping
        #[arg(short, long)]
        width: Option<usize>,
    },

    /// Submit puzzle answer
    Submit {
        /// Quest year
        #[arg(short, long, default_value_t = default_year().parse().unwrap())]
        year: i32,

        /// Quest day (1-20)
        #[arg(short, long)]
        day: i32,

        /// Quest part (1-3)
        #[arg(short, long)]
        part: i32,

        /// Answer to submit
        answer: String,
    },
}

impl Cli {
    pub fn validate(&self) -> Result<(), String> {
        match &self.command {
            Commands::Fetch { year, day, part, .. } => {
                validate_year(*year)?;
                validate_day(*day)?;
                validate_part(*part)?;
            }
            Commands::Read { year, day, .. } => {
                validate_year(*year)?;
                validate_day(*day)?;
            }
            Commands::Submit { year, day, part, .. } => {
                validate_year(*year)?;
                validate_day(*day)?;
                validate_part(*part)?;
            }
        }
        Ok(())
    }
}

fn validate_year(year: i32) -> Result<(), String> {
    if year < 2024 || year > 2030 {
        return Err(format!("Invalid year: {} (must be between 2024-2030)", year));
    }
    Ok(())
}

fn validate_day(day: i32) -> Result<(), String> {
    if !(1..=20).contains(&day) {
        return Err(format!("Invalid day: {} (must be 1-20)", day));
    }
    Ok(())
}

fn validate_part(part: i32) -> Result<(), String> {
    if !(1..=3).contains(&part) {
        return Err(format!("Invalid part: {} (must be 1-3)", part));
    }
    Ok(())
}
