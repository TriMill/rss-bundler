#![warn(clippy::pedantic)]

use std::{collections::{HashMap, HashSet}, thread, sync::{Mutex, Arc}, time::Duration, process::ExitCode, fs, panic::catch_unwind, io::{BufWriter, Write}};

use chrono::{DateTime, Utc};
use config::{Config, User};
use query::update_feeds;
use rss::Channel;
use crate::{junction::{bundle_rss, gen_status}, hooks::run_hook};

mod config;
mod query;
mod junction;
mod server;
mod hooks;

#[derive(Clone, Debug)]
pub struct Feed {
    channel: Option<Channel>,
    last_fetched: DateTime<Utc>,
    error_message: Option<String>,
}

pub struct State {
    rss: String,
    guids: HashSet<String>,
    feeds: HashMap<User, Feed>,
    status: Option<String>,
}

fn main() -> ExitCode {
    let config = match load_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            return 1.into()
        }
    };

    let guids = load_guids().unwrap_or_default();

    let state = State {
        rss: String::new(),
        guids,
        feeds: HashMap::new(),
        status: None,
    };

    let state = Arc::new(Mutex::new(state));

    let server_address = format!("{}:{}", config.host, config.port);
    println!("Starting server at {}", server_address);
    let server_threads = server::start(&server_address, config.worker_threads, state.clone());
    drop(server_threads);

    let sleep_duration = Duration::from_secs(60 * config.refresh_time);
    
    loop {
        let result = catch_unwind(|| {
            let mut guard = state.lock().unwrap();
            
            update_feeds(&mut guard.feeds, &config);
            let (hookdata, bundle) = bundle_rss(&mut guard, &config);
            let status = if config.status_page {
                Some(gen_status(&guard.feeds))
            } else { None };

            if let Some(hook) = &config.hook {
                run_hook(hook, hookdata).unwrap();
            }

            guard.status = status;
            guard.rss = bundle.to_string();
            save_guids(&guard.guids).unwrap();
            drop(guard);

        });
        if result.is_err() {
            eprintln!("Error occured white updating");
        } else {
            println!("Feeds updated");
        }
        thread::sleep(sleep_duration);
    }
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("config.json")?;
    let config: Config = serde_json::from_str(&content)?;
    Ok(config)
}

fn load_guids() -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("guids")?;
    Ok(content.split('\n').filter(|x| !x.is_empty()).map(str::to_owned).collect())
}

fn save_guids(guids: &HashSet<String>) -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::OpenOptions::new().create(true).write(true).open("guids")?;
    let mut writer = BufWriter::new(file);
    for guid in guids {
        writeln!(writer, "{}", guid)?;
    }
    Ok(())
}
