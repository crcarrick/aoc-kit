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
