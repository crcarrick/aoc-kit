use anyhow::Result;
use chrono::{Datelike, Utc};
use clap::Parser;

use crate::config::set_config;
use crate::utils::year_in_range;

#[derive(Debug, Parser)]
/// init the Advent of Code project
pub struct Command {
    #[arg(short = 'y', long = "year", value_parser = year_in_range, default_value_t = Utc::now().year())]
    current_year: i32,

    /// https://adventofcode.com session token
    #[arg(short, long)]
    token: String,
}

pub fn run_command(args: Command) -> Result<()> {
    let year = args.current_year.to_string();

    set_config(&year, &args.token)?;

    Ok(())
}
