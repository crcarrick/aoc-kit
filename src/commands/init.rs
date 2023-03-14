use anyhow::Result;
use chrono::{Datelike, Utc};
use clap::Parser;

use crate::config::set_config;

#[derive(Debug, Parser)]
/// init the Advent of Code project
pub struct Command {
    /// year to start with
    #[arg(short = 'y', long = "year", default_value_t = Utc::now().year().to_string())]
    current_year: String,
    /// https://adventofcode.com session token
    #[arg(short, long)]
    token: String,
}

pub fn run_command(args: Command) -> Result<()> {
    set_config(&args.current_year, &args.token)?;

    Ok(())
}
