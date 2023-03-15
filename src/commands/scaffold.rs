use std::fs;

use anyhow::{anyhow, Ok, Result};
use clap::Parser;
use collection_macros::hashmap;
use handlebars::Handlebars;
use serde_json::json;
use webbrowser;

use crate::{
    config::get_config,
    http::AOCClient,
    utils::{day_in_range, get_latest_day, update_workspace},
};

#[derive(Debug, Parser)]
/// create a new puzzle package
pub struct Command {
    #[arg(short, long, value_parser = day_in_range)]
    day: Option<i16>,

    /// open https://adventofcode.com to the created puzzle
    #[arg(short, long)]
    open: bool,
}

pub fn run_command(args: Command) -> Result<String> {
    // TODO: this method needs a refactor
    let cfg = get_config()?;
    let client = AOCClient::new()?;

    let base = match cfg.current_dir.to_str() {
        Some(b) => b,
        None => "",
    };
    let year = cfg.current_year;

    if base.is_empty() || year.is_empty() {
        return Err(anyhow!("Please run `aoc-kit init`"));
    }

    let handlebars = Handlebars::new();
    let cargo_str = include_str!("../templates/cargo.hbs");
    let part_str = include_str!("../templates/part.hbs");
    let lib_str = include_str!("../templates/lib.hbs");

    fs::create_dir_all(format!("{base}/{year}"))?;

    let day = get_latest_day(&year)?;
    let day = match args.day {
        Some(d) => d,
        None => match day {
            1 => 1,
            _ => day + 1,
        },
    }
    .to_string();

    let dir = format!("{base}/{year}/day_{:0>2}", day);

    fs::create_dir_all(format!("{dir}/src/bin"))?;

    let input = client
        .get_input(
            &format!("https://adventofcode.com/{year}/day/{day}/input"),
            hashmap! {},
        )?
        .text()?;
    fs::write(format!("{dir}/input.txt"), input)?;

    let cargo_tmpl =
        handlebars.render_template(cargo_str, &json!({ "day": format!("{:0>2}", day) }))?;
    fs::write(format!("{dir}/Cargo.toml"), cargo_tmpl)?;

    let lib_tmpl = handlebars.render_template(lib_str, &json!({}))?;
    fs::write(format!("{dir}/src/lib.rs"), lib_tmpl)?;

    for part in 'a'..='b' {
        let part = part.to_string();
        let part_tmpl = handlebars.render_template(
            part_str,
            &json!({ "day": format!("{:0>2}", day), "part": part }),
        )?;

        fs::write(format!("{dir}/src/bin/part_{part}.rs"), part_tmpl)?;
    }

    update_workspace(&format!("{year}/{:0>2}", day))?;

    if args.open {
        webbrowser::open(&format!("https://adventofcode.com/{year}/day/{day}"))?;
    }

    Ok(format!("Created puzzle {year}/{:0>2}", day))
}
