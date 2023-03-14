use std::fs;

use anyhow::{Ok, Result};
use clap::Parser;
use collection_macros::hashmap;
use handlebars::Handlebars;
use serde_json::json;
use webbrowser;

use crate::http::AdventClient;

#[derive(Debug, Parser)]
/// create a new puzzle package
pub struct Command {
    #[arg(short, long)]
    year: Option<String>,

    #[arg(short, long)]
    day: Option<String>,
}

#[tokio::main]
async fn get_input(url: &str, client: &AdventClient) -> Result<String> {
    let resp = client.get(url, hashmap! {})?.text().await?;

    Ok(resp)
}

// fn find_next_day() -> String {}

// fn find_next_year() -> String {}

pub fn run_command(args: Command, client: &AdventClient) -> Result<()> {
    let handlebars = Handlebars::new();
    let cargo_str = include_str!("../templates/cargo.hbs");
    let part_str = include_str!("../templates/part.hbs");
    let lib_str = include_str!("../templates/lib.hbs");

    let day = &args.day.unwrap_or("1".to_string());
    let year = &args.year.unwrap_or("2022".to_string());

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
