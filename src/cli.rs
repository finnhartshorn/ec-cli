use clap::{Parser, Subcommand};

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
        #[arg(short, long, default_value = "2024")]
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
        #[arg(short, long, default_value = "2024")]
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
        #[arg(short, long, default_value = "2024")]
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
