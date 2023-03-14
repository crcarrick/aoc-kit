use std::path::PathBuf;

use anyhow::Result;
use chrono::{Datelike, Utc};
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Store {
    pub session_token: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub current_year: String,
}

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
    let pwd = std::env::current_dir()?;
    let mut cfg_path = PathBuf::new();

    cfg_path.push(pwd);
    cfg_path.push(".aocrc");

    confy::store_path(
        cfg_path,
        Config {
            current_year: args.current_year,
        },
    )?;

    confy::store(
        "com.github.crcarrick.aockit",
        None,
        Store {
            session_token: args.token,
        },
    )?;

    Ok(())
}
