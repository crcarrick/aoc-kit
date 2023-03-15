use anyhow::Result;
use chrono::{Datelike, Utc};
use clap::Parser;

use crate::config::set_config;
use crate::utils::{init_workspace, year_in_range};

#[derive(Debug, Parser)]
/// init the Advent of Code project
pub struct Command {
    #[arg(short, long, value_parser = year_in_range)]
    year: Option<i32>,

    /// https://adventofcode.com session token
    #[arg(short, long)]
    token: String,
}

pub fn run_command(args: Command) -> Result<String> {
    let current_year = match args.year {
        Some(y) => y,
        // TODO: check if december, -> return year - 1 if not
        None => Utc::now().year(),
    }
    .to_string();
    let current_dir = std::env::current_dir()?;

    init_workspace(&current_dir)?;
    set_config(current_year.clone(), current_dir, &args.token)?;

    Ok(format!("year set to {current_year}"))
}
