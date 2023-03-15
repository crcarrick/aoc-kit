use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};
use chrono::{Datelike, Utc};
use serde::{Deserialize, Serialize};

use crate::config::get_config;

pub fn day_in_range(day: &str) -> Result<i16> {
    let day_range = 1..=25;
    let day = day
        .parse()
        .map_err(|_| anyhow!("`{}` is not a day number", day))?;

    if day_range.contains(&day) {
        Ok(day)
    } else {
        Err(anyhow!(
            "Day not in range {}-{}",
            day_range.start(),
            day_range.end()
        ))
    }
}

pub fn year_in_range(year: &str) -> Result<i32> {
    let year_range = 2016..=Utc::now().year();
    let year = year
        .parse()
        .map_err(|_| anyhow!("`{}` is not a year number", year))?;

    if year_range.contains(&year) {
        Ok(year)
    } else {
        Err(anyhow!(
            "Year not in range {}-{}",
            year_range.start(),
            year_range.end()
        ))
    }
}

pub fn get_latest_day(current_year: &str) -> Result<i16> {
    let cfg = get_config()?;
    let mut dir = cfg.current_dir.clone();

    dir.push(current_year);

    let paths = fs::read_dir(&dir)?;

    let mut max_day: Option<i16> = None;
    for entry in paths {
        let entry = entry?;
        if let Some(day) = extract_day_from_filename(&entry.file_name()) {
            max_day = Some(max_day.map_or(day, |max| max.max(day)));
        }
    }

    Ok(max_day.unwrap_or(1))
}

fn extract_day_from_filename(filename: &std::ffi::OsString) -> Option<i16> {
    let file_name_str = filename.to_str()?;
    let day_str = file_name_str.split_once("_")?.1;
    day_str.parse::<i16>().ok()
}

#[derive(Debug, Deserialize, Serialize)]
struct CargoWorkspace {
    members: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct CargoToml {
    workspace: CargoWorkspace,
}

pub fn init_workspace(current_dir: &PathBuf) -> Result<()> {
    let mut path = current_dir.clone();

    path.push("Cargo.toml");

    let cargo = CargoToml {
        workspace: CargoWorkspace { members: vec![] },
    };

    fs::write(path, toml::to_string(&cargo)?)?;

    Ok(())
}

pub fn update_workspace(pkg: &str) -> Result<()> {
    let cfg = get_config()?;
    let mut path = cfg.current_dir.clone();

    path.push("Cargo.toml");

    let cargo = fs::read_to_string(&path)?;
    let mut cargo: CargoToml = toml::from_str(&cargo)?;

    cargo.workspace.members.push(String::from(pkg));

    fs::write(&path, toml::to_string(&cargo)?)?;

    Ok(())
}
