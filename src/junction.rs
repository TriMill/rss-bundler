use std::collections::HashMap;

use chrono::{DateTime, SubsecRound};
use rss::Channel;
use strfmt::strfmt;

use crate::{Feed, State};
use crate::config::{Config, User};
use crate::hooks::HookData;

pub fn bundle_rss(state: &mut State, config: &Config) -> (Vec<HookData>, Channel) {
    let mut bundle = Channel::default();
    bundle.set_title(&config.title);
    bundle.set_link(&config.link);
    bundle.description = config.description.clone();
    bundle.set_generator(Some("RSS Bundler".into()));
    let mut hookdata = Vec::new();
    let mut most_recent_date = None;
    for (user, feed) in &state.feeds {
        if let Some(channel) = &feed.channel {
            for item in channel.items() {
                if let Some(pub_date) = &item.pub_date {
                    if let Ok(date) = DateTime::parse_from_rfc2822(pub_date) {
                        match most_recent_date {
                            None => most_recent_date = Some(date),
                            Some(d) if date > d => most_recent_date = Some(date),
                            _ => ()
                        }
                    }
                }
                let mut item = item.clone();
                if item.author.is_none() {
                    item.set_author(user.name.clone());
                }
                let item_title = {
                    let title = item.title.as_ref().unwrap_or(&config.default_title);
                    let mut args = HashMap::new();
                    args.insert("title".into(), title);
                    args.insert("name".into(), &user.name);
                    match strfmt(&config.title_format, &args) {
                        Ok(res) => res,
                        Err(e) => {
                            eprintln!("Format string error: {}. Using default format string instead.", e);
                            format!("[{}] {}", title, user.name)
                        }
                    }
                };
                if let Some(guid) = &item.guid {
                    if !state.guids.contains(&guid.value) {
                        state.guids.insert(guid.value.clone());
                            
                        let data = HookData {
                            title: item.title.as_ref().unwrap_or(&config.default_title).clone(),
                            title_fmt: item_title.clone(),
                            author: item.author.clone().unwrap(),
                            link: item.link.clone().unwrap_or_default(),
                            guid: item.guid.clone().map(|g| g.value).unwrap_or_default(),
                            pub_date: item.pub_date.clone().unwrap_or_default(),
                        };

                        hookdata.push(data);
                    }
                }
                item.set_title(item_title);
                bundle.items.push(item.clone());
            }
        }
    }
    if let Some(date) = most_recent_date {
        bundle.set_pub_date(date.to_rfc2822());
    }
    (hookdata, bundle)
}

pub fn gen_status(feeds: &HashMap<User, Feed>) -> String {
    let max_user_length = feeds.iter()
        .map(|(user, _)| user.name.len())
        .max().unwrap_or(0).max(4);
    let max_timestamp_length = feeds.iter()
        .map(|(_, feed)| feed.last_fetched.round_subsecs(0).to_rfc3339().len())
        .max().unwrap_or(0).max(12);
    let mut lines = vec![
        format!("{:w_user$}\t{:w_time$}\t{:6}\t{}", 
            "USER", "LAST SUCCESS", "STATUS", "ERROR",
            w_user=max_user_length, w_time=max_timestamp_length)
    ];
    for (user, feed) in feeds {
        let (status, error) = match &feed.error_message {
            Some(e) => ("ERROR", e.as_str()),
            None => ("OK", ""),
        };
        lines.push(format!("{:w_user$}\t{:w_time$}\t{:6}\t{}", 
            user.name, feed.last_fetched.round_subsecs(0).to_rfc3339(), status, error,
            w_user=max_user_length, w_time=max_timestamp_length));
    }
    lines.join("\n")
}
