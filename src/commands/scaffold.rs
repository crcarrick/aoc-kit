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
    utils::{day_in_range, nearest_aoc_dir},
};

#[derive(Debug, Parser)]
/// create a new puzzle package
pub struct Command {
    #[arg(short, long, value_parser = day_in_range)]
    day: Option<i16>,

    /// open https://adventofcode.com to the created puzzle
    #[arg(short, long, default_value_t = true)]
    open: bool,
}

fn find_next_day(current_year: &str) -> Result<i16> {
    let pwd = std::env::current_dir()?;
    let mut dir = nearest_aoc_dir(pwd)?;

    dir.push(current_year);

    let paths = std::fs::read_dir(dir)?;
    let mut paths = paths.filter_map(|p| p.ok()).collect::<Vec<_>>();

    paths.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    let last = paths.last();

    // TODO: good god there has to be a better way to do this
    if let Some(l) = last {
        if let Some(f) = l.file_name().to_str() {
            if let Some((_, day)) = f.split_once("_") {
                if let Some(d) = day.parse::<i16>().ok() {
                    if d == 25 {
                        return Err(anyhow!("25 is the last day of the advent calendar"));
                    }

                    return Ok(d + 1);
                }
            }
        }
    }

    Ok(1)
}

pub fn run_command(args: Command) -> Result<()> {
    // TODO: this method needs a refactor
    let client = AOCClient::new()?;
    let cfg = get_config()?;

    if cfg.current_year.is_empty() {
        return Err(anyhow!("no year found, please run `aoc-kit init`"));
    }

    let handlebars = Handlebars::new();
    let cargo_str = include_str!("../templates/cargo.hbs");
    let part_str = include_str!("../templates/part.hbs");
    let lib_str = include_str!("../templates/lib.hbs");

    let year = cfg.current_year;
    fs::create_dir_all(&year)?;

    let day = match args.day {
        Some(d) => d.to_string(),
        None => find_next_day(&year)?.to_string(),
    };

    let dir = format!("{year}/day_{:0>2}", day);

    fs::create_dir_all(format!("{dir}/src/bin"))?;

    let input = client
        .get(
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
        let part_tmpl =
            handlebars.render_template(part_str, &json!({ "day": day, "part": part }))?;

        fs::write(format!("{dir}/src/bin/part_{part}.rs"), part_tmpl)?;
    }

    if args.open {
        webbrowser::open(&format!("https://adventofcode.com/{year}/day/{day}"))?;
    }

    Ok(())
}
