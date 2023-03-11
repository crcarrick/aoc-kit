use std::fs;

use anyhow::Result;
use clap::Parser;
use handlebars::Handlebars;
use serde_json::json;
use webbrowser;

#[derive(Parser, Debug)]
/// Scaffold a new Advent of Code year
pub struct Command {
    /// The year to scaffold
    #[arg(short = 'y', long = "year")]
    year: String,
}

fn get_url(year: &str) -> String {
    String::from("https://adventofcode.com/") + year
}

fn scaffold_solution(year: &str) -> Result<()> {
    let handlebars = Handlebars::new();
    let contents = handlebars.render_template(
        include_str!("../templates/solution.hbs"),
        &json!({ "year": year }),
    )?;

    fs::write("solution.rs", contents)?;

    Ok(())
}

pub fn run_command(command: Command) -> Result<()> {
    scaffold_solution(&command.year)?;

    webbrowser::open(&get_url(&command.year))?;

    Ok(())
}
