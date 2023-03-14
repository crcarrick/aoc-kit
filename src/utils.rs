use anyhow::{anyhow, Result};

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
