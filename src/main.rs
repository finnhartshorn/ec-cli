mod cli;
mod client;
mod crypto;
mod display;
mod error;
mod models;
mod storage;

use clap::Parser;
use log::{error, info};

use crate::cli::{Cli, Commands};
use crate::client::EcClient;
use crate::storage::Storage;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Set up logging
    let log_level = if cli.debug {
        "debug"
    } else if cli.quiet {
        "error"
    } else {
        "info"
    };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    // Validate arguments
    if let Err(e) = cli.validate() {
        error!("{}", e);
        std::process::exit(1);
    }

    // Execute command
    let result = match cli.command {
        Commands::Fetch {
            year,
            day,
            part,
            description_only,
            input_only,
            description_path,
            input_path,
        } => {
            handle_fetch(
                cli.base_path.clone(),
                year,
                day,
                part,
                description_only,
                input_only,
                description_path,
                input_path,
            )
            .await
        }
        Commands::Read { year, day, width } => {
            handle_read(cli.base_path.clone(), year, day, width).await
        }
        Commands::Submit {
            year,
            day,
            part,
            answer,
        } => {
            handle_submit(year, day, part, &answer).await
        }
    };

    if let Err(e) = result {
        error!("{}", e);
        std::process::exit(1);
    }
}

async fn handle_fetch(
    base_path: Option<String>,
    year: i32,
    day: i32,
    part: i32,
    description_only: bool,
    input_only: bool,
    description_path: Option<String>,
    input_path: Option<String>,
) -> error::Result<()> {
    let mut client = EcClient::new()?;

    // Build storage with custom paths
    let mut storage = Storage::new(base_path.map(|p| p.into()));

    if let Some(desc_path) = description_path {
        storage = storage.with_description_path(desc_path.into());
    }

    if let Some(inp_path) = input_path {
        storage = storage.with_input_path(inp_path.into());
    }

    // Fetch description (unless input_only)
    if !input_only {
        let description = client.fetch_description(year, day).await?;
        let path = storage.save_description(year, day, &description)?;
        info!("Description saved to {:?}", path);

        // Extract and save samples
        let samples = display::extract_samples(&description);
        if !samples.is_empty() {
            info!("Found {} sample(s)", samples.len());
            for (i, sample) in samples.iter().enumerate() {
                // Save samples with part index (assuming part 1 for samples)
                let sample_part = i as i32 + 1;
                if sample_part <= 3 {
                    let path = storage.save_sample(year, day, sample_part, sample)?;
                    info!("Sample {} saved to {:?}", sample_part, path);
                }
            }
        }
    }

    // Fetch input (unless description_only)
    if !description_only {
        let input = client.fetch_input(year, day, part).await?;
        let path = storage.save_input(year, day, part, &input)?;
        info!("Input saved to {:?}", path);
    }

    Ok(())
}

async fn handle_read(base_path: Option<String>, year: i32, day: i32, width: Option<usize>) -> error::Result<()> {
    let storage = Storage::new(base_path.map(|p| p.into()));

    // Check if description exists locally and if it needs updating
    let description = if storage.has_description(year, day) {
        let cached = storage.load_description(year, day)?;

        // Check if we might have new parts available
        let mut client = EcClient::new()?;
        let keys = client.fetch_quest_keys(year, day).await?;

        // Count how many parts we have keys for
        let available_parts = 1 + keys.key2.is_some() as usize + keys.key3.is_some() as usize;

        // Count how many PART markers are in the cached description
        // Part 1 has no marker, so parts 2 and 3 add markers
        let cached_parts = 1 + cached.matches(" PART 2 ").count() + cached.matches(" PART 3 ").count();

        if cached_parts < available_parts {
            info!("New parts unlocked, re-fetching description...");
            let desc = client.fetch_description(year, day).await?;
            storage.save_description(year, day, &desc)?;
            desc
        } else {
            info!("Reading description from local storage...");
            cached
        }
    } else {
        info!("Description not found locally, fetching...");
        let mut client = EcClient::new()?;
        let desc = client.fetch_description(year, day).await?;
        storage.save_description(year, day, &desc)?;
        desc
    };

    // Determine terminal width
    let display_width = width.unwrap_or_else(|| {
        term_size::dimensions()
            .map(|(w, _)| w)
            .unwrap_or(80)
    });

    // Convert HTML to text and display
    let text = display::html_to_text(&description, display_width);
    println!("{}", text);

    Ok(())
}

async fn handle_submit(year: i32, day: i32, part: i32, answer: &str) -> error::Result<()> {
    let client = EcClient::new()?;
    let response = client.submit_answer(year, day, part, answer).await?;

    // Display formatted response
    let output = display::format_submit_response(&response);
    println!("{}", output);

    Ok(())
}
