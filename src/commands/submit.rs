use anyhow::{anyhow, Result};
use chrono::{Datelike, Utc};
use clap::{Parser, ValueEnum};
use collection_macros::hashmap;

use crate::http::AOCClient;
use crate::utils::{day_in_range, year_in_range};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Part {
    A,
    B,
}

#[derive(Parser, Debug)]
/// submit an answer for a puzzle
pub struct Command {
    answer: String,

    #[arg(short, long, value_parser = year_in_range, default_value_t = Utc::now().year())]
    year: i32,

    #[arg(short, long, value_parser = day_in_range, default_value_t = 1)]
    day: i16,

    #[arg(short, long, value_enum, default_value_t = Part::A)]
    part: Part,

    /// open adventofcode.com to the next puzzle
    #[arg(short, long, default_value_t = true)]
    open: bool,
}

pub fn run_command(args: Command) -> Result<()> {
    let client = AOCClient::new()?;

    let year = &args.year;
    let day = &args.day;
    let part = match args.part {
        Part::A => "a",
        Part::B => "b",
    };

    // TODO: It would be really nice if we could _run_ the solutions
    //       and then submit the output
    client.post(
        &format!("https://adventofcode/{year}/day/{day}/answer"),
        hashmap! {
            "answer" => args.answer.as_str(),
            "level" => part,
        },
    )?;

    if args.open {
        webbrowser::open(&format!("https://adventofcode.com/{year}/day/{}", day + 1))?;
    }

    Ok(())
}
