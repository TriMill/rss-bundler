use std::{collections::HashMap, time::Duration, str::FromStr};

use chrono::{Utc, TimeZone};
use reqwest::blocking::Client;
use rss::Channel;

use crate::Feed;
use crate::config::{User, Config};

pub fn update_feeds(feeds: &mut HashMap<User, Feed>, config: &Config) {
    let client = Client::new();
    for user in &config.users {
        let feed = match feeds.get_mut(user) {
            Some(feed) => feed,
            None => {
                let feed = Feed {
                    channel: None,
                    error_message: None,
                    last_fetched: Utc.ymd(1970, 1, 1).and_hms(0, 0, 0)
                };
                feeds.insert(user.clone(), feed);
                feeds.get_mut(user).unwrap()
            }
        };
        let res = client.get(&user.rss)
            .timeout(Duration::from_secs(5))
            .send();
        let time = Utc::now();
        match res {
            Ok(res) if res.status().is_success() => match res.text() {
                Ok(text) => match Channel::from_str(&text) {
                    Ok(channel) => { 
                        feed.last_fetched = time;
                        feed.error_message = None;
                        feed.channel = Some(channel);
                    },
                    Err(e) => feed.error_message = Some(e.to_string())
                },
                Err(e) => feed.error_message = Some(e.to_string()),
            },
            Ok(res) => match res.status().canonical_reason() {
                Some(reason) => feed.error_message = Some(format!("HTTP {} ({})", res.status().as_str(), reason)),
                None => feed.error_message = Some(format!("HTTP {} (unknown)", res.status().as_str())),
            },
            Err(e) => feed.error_message = Some(e.to_string()),
        }
    }
}