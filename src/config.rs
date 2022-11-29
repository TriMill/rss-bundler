use serde::{Serialize, Deserialize};

fn const_true() -> bool { true }
fn default_timeout() -> u64 { 60 }
fn default_port() -> u16 { 4400 }
fn default_host() -> String { "127.0.0.1".into() }
fn default_worker_threads() -> usize { 4 }
fn default_title_format() -> String { "[{name}] {title}".into() }
fn default_default_title() -> String { "<untitled>".into() }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub title: String,
    pub link: String,
    #[serde(default)]
    pub description: String,
    #[serde(default="default_default_title")]
    pub default_title: String,

    #[serde(default="default_timeout")]
    pub refresh_time: u64,
    #[serde(default="const_true")]
    pub status_page: bool,
    #[serde(default="default_title_format")]
    pub title_format: String,
    #[serde(default="default_worker_threads")]
    pub worker_threads: usize,
    #[serde(default="default_port")]
    pub port: u16,
    #[serde(default="default_host")]
    pub host: String,

    #[serde(default)]
    pub hook: Option<String>,

    pub users: Vec<User>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub rss: String,
}
