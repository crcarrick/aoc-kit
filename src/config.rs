use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub current_dir: PathBuf,
    pub current_year: String,
    pub session_token: String,
}

const APP_NAME: &str = "com.github.crcarrick.aockit";

pub fn get_config() -> Result<Config> {
    let cfg: Config = confy::load(APP_NAME, None)?;

    Ok(cfg)
}

pub fn set_config(current_year: String, current_dir: PathBuf, session_token: &str) -> Result<()> {
    confy::store(
        APP_NAME,
        None,
        Config {
            current_dir,
            current_year,
            session_token: String::from(session_token),
        },
    )?;

    Ok(())
}
