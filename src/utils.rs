use std::path::PathBuf;

use anyhow::{anyhow, Result};
use chrono::{Datelike, Utc};

pub fn day_in_range(day: &str) -> Result<i16> {
    let day_range = 1..=25;
    let day = day
        .parse()
        .map_err(|_| anyhow!("`{}` is not a day number", day))?;

    if day_range.contains(&day) {
        Ok(day)
    } else {
        Err(anyhow!(
            "day not in range {}-{}",
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
            "year not in range {}-{}",
            year_range.start(),
            year_range.end()
        ))
    }
}

pub fn nearest_aoc_dir(dir: PathBuf) -> Result<PathBuf> {
    let paths = std::fs::read_dir(&dir)?;

    let found = paths
        .filter_map(|p| p.ok())
        .find(|p| {
            if let Some(m) = p.metadata().ok() {
                return m.is_file() && p.file_name() == "aoc.yml";
            }

            return false;
        })
        .is_some();

    if found {
        return Ok(dir);
    }

    if let Some(p) = dir.parent() {
        return nearest_aoc_dir(p.to_path_buf());
    }

    return Err(anyhow!("could not find aoc.yml"));
}

pub fn get_latest_day(current_year: &str) -> Result<i16> {
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
                    return Ok(d);
                }
            }
        }
    }

    Ok(1)
}
