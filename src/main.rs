use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;
mod http;

use http::AdventClient;

#[derive(Debug, Subcommand)]
enum Commands {
    Init(commands::init::Command),
    Scaffold(commands::scaffold::Command),
    Submit(commands::submit::Command),
}

#[derive(Debug, Parser)]
#[command(
    author = "Chris Carrick <chris@crcarrick.dev>",
    version = "0.1.0",
    about = "Kit for solving Advent of Code puzzles"
)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> Result<()> {
    let cli = CLI::parse();
    let cfg: commands::init::Store = confy::load("com.github.crcarrick.aockit", None)?;
    let client = AdventClient::new(&cfg.session_token)?;

    match cli.command {
        Commands::Init(args) => {
            match commands::init::run_command(args) {
                Err(e) => {
                    eprintln!("{:?}", e);
                    // TODO: Handle the error
                }
                Ok(()) => {
                    // TODO: Some output
                }
            };
        }
        Commands::Scaffold(args) => {
            match commands::scaffold::run_command(args, &client) {
                Err(e) => {
                    eprintln!("{:?}", e);
                    // TODO: Handle the error
                }
                Ok(()) => {
                    // TODO: Some output
                }
            };
        }
        Commands::Submit(args) => {
            match commands::submit::run_command(args, &client) {
                Err(e) => {
                    eprintln!("{:?}", e);
                    // TODO: Handle the error
                }
                Ok(()) => {
                    // TODO: Some output
                }
            };
        }
    }

    Ok(())
}
