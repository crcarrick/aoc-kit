use std::{collections::HashMap, sync::Arc};

use anyhow::{anyhow, Result};
use reqwest::{
    blocking::{Client, Response},
    cookie::Jar,
    Url,
};

use crate::config::get_config;

#[derive(Debug)]
pub struct AOCClient {
    client: Client,
}

#[derive(Debug, Default)]
struct Params {
    params: Vec<(String, String)>,
}

impl TryFrom<HashMap<&str, &str>> for Params {
    type Error = anyhow::Error;

    fn try_from(value: HashMap<&str, &str>) -> Result<Self> {
        if value.is_empty() {
            return Ok(Params::default());
        }

        return Ok(Params {
            params: value
                .into_iter()
                .map(|(key, val)| (key.to_string(), val.to_string()))
                .collect(),
        });
    }
}

impl AOCClient {
    pub fn new() -> Result<AOCClient> {
        let cfg = get_config()?;
        let token = cfg.session_token;

        if token.is_empty() {
            // TODO: better error message
            return Err(anyhow!("Please run `aockit init`"));
        }

        let jar = Jar::default();
        let url = "https://adventofcode.com".parse::<Url>()?;
        let cookie = format!("session={token}; Domain=adventofcode.com");

        jar.add_cookie_str(&cookie, &url);

        let client = Client::builder().cookie_provider(Arc::new(jar)).build()?;

        return Ok(AOCClient { client });
    }

    pub fn get_input(&self, url: &str, params: HashMap<&str, &str>) -> Result<Response> {
        let params: Params = params.try_into()?;
        let url = Url::parse_with_params(&url, &params.params)?;
        let resp = self.client.get(url).send()?;

        match resp.status() {
            reqwest::StatusCode::OK => Ok(resp),
            _ => Err(anyhow!("Status code {}", resp.status())),
        }
    }

    pub fn submit_answer(&self, url: &str, params: HashMap<&str, &str>) -> Result<Response> {
        let url = Url::parse(&url)?;
        let resp = self.client.post(url).form(&params).send()?;

        match resp.status() {
            reqwest::StatusCode::OK => Ok(resp),
            _ => Err(anyhow!("Status code {}", resp.status())),
        }
    }
}
