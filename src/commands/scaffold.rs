use std::fs;

use anyhow::{Ok, Result};
use clap::Parser;
use collection_macros::hashmap;
use handlebars::Handlebars;
use serde_json::json;
use webbrowser;

use crate::{config::get_config, http::AOCClient};

#[derive(Debug, Parser)]
/// create a new puzzle package
pub struct Command {
    #[arg(short, long, default_value_t = 1)]
    day: i16,
}

#[tokio::main]
async fn get_input(url: &str) -> Result<String> {
    let client = AOCClient::new()?;
    let resp = client.get(url, hashmap! {})?.text().await?;

    Ok(resp)
}

// fn find_next_day() -> String {}

// fn find_next_year() -> String {}

pub fn run_command(args: Command) -> Result<()> {
    let cfg = get_config()?;

    let handlebars = Handlebars::new();
    let cargo_str = include_str!("../templates/cargo.hbs");
    let part_str = include_str!("../templates/part.hbs");
    let lib_str = include_str!("../templates/lib.hbs");

    let day = args.day.to_string();
    let year = cfg.current_year;

    fs::create_dir_all(format!("{year}/{:0>2}", day))?;

    // let day = day.to_string();
    // let dir = format!("{year}/day_{:0>2}", day);
    // let input = get_input(
    //     &format!("https://adventofcode.com/{year}/day/{day}/input"),
    //     client,
    // )?;

    // let cargo_tmpl =
    //     handlebars.render_template(cargo_str, &json!({ "day": format!("{:0>2}", day) }))?;
    // let lib_tmpl = handlebars.render_template(lib_str, &json!({}))?;

    // fs::create_dir_all(format!("{dir}/src/bin"))?;
    // fs::write(format!("{dir}/input.txt"), input)?;
    // fs::write(format!("{dir}/Cargo.toml"), cargo_tmpl)?;
    // fs::write(format!("{dir}/src/lib.rs"), lib_tmpl)?;

    // for part in 'a'..='b' {
    //     let part = part.to_string();
    //     let part_tmpl =
    //         handlebars.render_template(part_str, &json!({ "day": day, "part": part }))?;

    //     fs::write(format!("{dir}/src/bin/part_{part}.rs"), part_tmpl)?;
    // }

    webbrowser::open(&format!("https://adventofcode.com/{year}"))?;

    Ok(())
}
