use std::{env::current_dir, path::PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
struct AppConfig {
    session_token: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct LocalConfig {
    current_year: String,
}

#[derive(Debug)]
pub struct Config {
    pub current_year: String,
    pub session_token: String,
}

pub fn get_local_config_path() -> Result<PathBuf> {
    let pwd = current_dir()?;
    let mut path = PathBuf::new();

    path.push(pwd);
    path.push("aoc.yml");

    Ok(path)
}

const APP_NAME: &str = "com.github.crcarrick.aockit";

pub fn get_config() -> Result<Config> {
    let path = get_local_config_path()?;
    let app_cfg: AppConfig = confy::load(APP_NAME, None)?;
    let local_cfg: LocalConfig = confy::load_path(path)?;

    Ok(Config {
        current_year: local_cfg.current_year,
        session_token: app_cfg.session_token,
    })
}

pub fn set_config(current_year: &str, session_token: &str) -> Result<()> {
    confy::store(
        APP_NAME,
        None,
        AppConfig {
            session_token: String::from(session_token),
        },
    )?;

    let path = get_local_config_path()?;

    confy::store_path(
        path,
        LocalConfig {
            current_year: String::from(current_year),
        },
    )?;

    Ok(())
}
