use anyhow::Result;
use clap::Parser;

mod commands;

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

    match cli {
        CLI::Login(c) => {
            match commands::login::run_command(c) {
                Err(_e) => {
                    // TODO: Handle the error
                }
                Ok(()) => {
                    // TODO: Some output
                }
            };
        }
        CLI::Scaffold(c) => {
            match commands::scaffold::run_command(c) {
                Err(e) => {
                    eprintln!("{:?}", e);
                    // TODO: Handle the error
                }
                Ok(()) => {
                    // TODO: Some output
                }
            };
        }
        CLI::Submit(c) => {
            match commands::submit::run_command(c) {
                Err(e) => {
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
