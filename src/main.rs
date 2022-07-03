#![warn(clippy::pedantic)]

use std::{collections::HashMap, thread, sync::{Mutex, Arc}, time::Duration, env::args, process::ExitCode, fs};

use chrono::{DateTime, Utc};
use config::Config;
use query::update_feeds;
use rss::Channel;
use crate::{junction::{bundle_rss, gen_status}};

mod config;
mod query;
mod junction;
mod server;

#[derive(Clone, Debug)]
pub struct Feed {
    channel: Option<Channel>,
    last_fetched: DateTime<Utc>,
    error_message: Option<String>,
}

pub struct State {
    rss: String,
    status: Option<String>,
}

fn main() -> ExitCode {
    let mut args = args();
    let exe = args.next();
    let config_file = args.next();
    let config_file = match &config_file {
        Some(s) if s == "--help" => { 
            eprintln!(
                "Usage: {} <config-file>\nDocumentation available at https://github.com/trimill/rss-bundler", 
                exe.unwrap_or_else(|| "rssbundler".into())); 
            return 0.into()
        }
        Some(file) => file,
        None => { 
            eprintln!("No config file provided."); 
            return 1.into()
        }
    };
    let config = match load_config(config_file) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            return 1.into()
        }
    };
    let mut feeds = HashMap::new();

    let state = State {
        rss: "".into(),
        status: None,
    };

    let state = Arc::new(Mutex::new(state));

    let server_address = format!("{}:{}", config.host, config.port);
    println!("Starting server at {}", server_address);
    let server_threads = server::start(&server_address, config.worker_threads, state.clone());
    drop(server_threads);

    let sleep_duration = Duration::from_secs(60 * config.refresh_time);
    
    loop {
        update_feeds(&mut feeds, &config);
        let bundle = bundle_rss(&feeds, &config);
        let status = if config.status_page {
            Some(gen_status(&feeds))
        } else { None };

        let mut guard = state.lock().unwrap();
        guard.status = status;
        guard.rss = bundle.to_string();
        drop(guard);

        println!("Feeds updated");

        thread::sleep(sleep_duration);
    }
}

fn load_config(config_file: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(config_file)?;
    let config: Config = serde_json::from_str(&content)?;
    Ok(config)
}