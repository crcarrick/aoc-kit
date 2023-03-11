use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
/// Run and submit an answer for a puzzle
pub struct Command {
    /// The puzzle day
    #[arg(short = 'd', long = "day")]
    day: String,

    /// The puzzle part (a, b)
    #[arg(short = 'p', long = "part")]
    part: String,
}

fn get_url(day: &str, part: &str) -> String {
    // TODO: Get the year somehow
    String::from("https://adventofcode.com/2022/day/") + day + "#part" + part
}

// TODO: Get access to Config struct here
pub fn run_command(command: Command) -> Result<()> {
    // TODO: Submit the answer using reqwest
    //          If Success: scaffold the new puzzle
    //          If Err: some message

    // TODO: Open the next puzzle with webbrowser
    webbrowser::open(&get_url(&command.day, &command.part))?;

    Ok(())
}
