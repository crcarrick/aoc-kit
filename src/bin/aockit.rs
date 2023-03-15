use anyhow::Result;
use clap::{Parser, Subcommand};

use aoc_kit::commands;

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
    about = "Utilities for solving Advent of Code puzzles"
)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> Result<()> {
    let cli = CLI::parse();

    match cli.command {
        Commands::Init(args) => {
            match commands::init::run_command(args) {
                Err(e) => {
                    eprintln!("{:?}", e);
                    // TODO: handle the error
                }
                Ok(r) => {
                    println!("{}", r);
                }
            };
        }
        Commands::Scaffold(args) => {
            match commands::scaffold::run_command(args) {
                Err(e) => {
                    eprintln!("{:?}", e);
                    // TODO: handle the error
                }
                Ok(r) => {
                    println!("{}", r);
                }
            };
        }
        Commands::Submit(args) => {
            match commands::submit::run_command(args) {
                Err(e) => {
                    eprintln!("{:?}", e);
                    // TODO: handle the error
                }
                Ok(r) => {
                    println!("{:?}", r);
                }
            };
        }
    }

    Ok(())
}
