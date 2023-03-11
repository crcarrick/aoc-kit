use anyhow::Result;
use clap::Parser;

mod commands;
mod http;

use http::AdventClient;

#[derive(Parser, Debug)]
#[command(
    author = "Chris Carrick <chris@crcarrick.dev>",
    version = "0.1.0",
    about = "Kit for solving Advent of Code puzzles"
)]
enum CLI {
    Login(commands::login::Command),
    Scaffold(commands::scaffold::Command),
    Submit(commands::submit::Command),
}

fn main() -> Result<()> {
    let cli = CLI::parse();
    let cfg: commands::login::Config = confy::load("com.github.crcarrick.aockit", None)?;
    let client = AdventClient::new(&cfg.session_token)?;

    match cli {
        CLI::Login(command) => {
            match commands::login::run_command(command) {
                Err(e) => {
                    eprintln!("{:?}", e);
                    // TODO: Handle the error
                }
                Ok(()) => {
                    // TODO: Some output
                }
            };
        }
        CLI::Scaffold(command) => {
            match commands::scaffold::run_command(command, &client) {
                Err(e) => {
                    eprintln!("{:?}", e);
                    // TODO: Handle the error
                }
                Ok(()) => {
                    // TODO: Some output
                }
            };
        }
        CLI::Submit(command) => {
            match commands::submit::run_command(command, &client) {
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
