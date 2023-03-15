use anyhow::Result;
use clap::{Parser, ValueEnum};
use collection_macros::hashmap;

use crate::config::get_config;
use crate::http::AOCClient;
use crate::utils::{day_in_range, get_latest_day, year_in_range};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Part {
    A,
    B,
}

#[derive(Parser, Debug)]
/// submit an answer for a puzzle
pub struct Command {
    answer: String,

    #[arg(short, long, value_parser = year_in_range)]
    year: Option<i32>,

    #[arg(short, long, value_parser = day_in_range)]
    day: Option<i16>,

    #[arg(short, long, value_enum, default_value_t = Part::A)]
    part: Part,
}

pub fn run_command(args: Command) -> Result<String> {
    let cfg = get_config()?;
    let client = AOCClient::new()?;

    let year = &cfg.current_year;
    let day = match args.day {
        Some(d) => d,
        None => get_latest_day(&year)?,
    };
    let level = match args.part {
        Part::A => "1",
        Part::B => "2",
    };

    // TODO: It would be really nice if we could _run_ the solutions
    //       and then submit the output
    let resp = client.submit_answer(
        &format!("https://adventofcode.com/{year}/day/{day}/answer"),
        hashmap! {
            "answer" => args.answer.as_str(),
            "level" => level,
        },
    )?;
    let text = resp.text()?;

    if text.contains("That's not the right answer") {
        if text.contains("too low") {
            return Ok("That's not the right answer. Your answer is too low".to_string());
        } else {
            return Ok("That's not the right answer. Your answer is too high".to_string());
        }
    }

    Ok("That answer is correct!".to_string())
}
