use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub session_token: String,
}

#[derive(Parser, Debug)]
/// Authenticate with https://adventofcode.com
pub struct Command {
    /// Session token
    #[arg(short = 't', long = "token")]
    token: String,
}

pub fn run_command(command: Command) -> Result<()> {
    confy::store(
        "com.github.crcarrick.aockit",
        None,
        Config {
            session_token: command.token,
        },
    )?;

    Ok(())
}
