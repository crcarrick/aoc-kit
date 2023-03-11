use anyhow::Result;
use clap::Parser;
use collection_macros::hashmap;

use crate::AdventClient;

#[derive(Parser, Debug)]
/// Run and submit an answer for a puzzle
pub struct Command {
    /// The puzzle answer
    answer: String,

    /// The puzzle year
    #[arg(short = 'y', long = "year", default_value_t = String::from("2022"))]
    year: String,

    /// The puzzle day
    #[arg(short = 'd', long = "day")]
    day: String,

    /// The puzzle part (a, b)
    #[arg(short = 'p', long = "part")]
    part: String,

    /// Open adventofcode.com to the specified puzzle
    #[arg(short = 'o', long = "open")]
    open: bool,
}

// TODO: Get access to Config struct here
pub fn run_command(command: Command, client: &AdventClient) -> Result<()> {
    let year = &command.year;
    let day = &command.day;
    let part = &command.day;

    // TODO: It would be really nice if we could _run_ the solutions
    //       and then submit the output but I'm a noob and dont' know how
    //       how to accomplish that yet
    client.post(
        &format!("https://adventofcode/{year}/day/{day}/answer"),
        hashmap! {
            "answer" => command.answer.as_str(),
            "level" => command.part.as_str()
        },
    )?;

    if command.open {
        webbrowser::open(&format!(
            "https://adventofcode.com/{year}/day/{day}#part{part}"
        ))?;
    }

    Ok(())
}
