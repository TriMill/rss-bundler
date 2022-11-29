use std::process::Command;

pub struct HookData {
    pub title: String,
    pub title_fmt: String,
    pub author: String,
    pub link: String,
    pub guid: String,
    pub pub_date: String,
}

pub fn run_hook(hook: &str, hookdata: Vec<HookData>) -> Result<(), std::io::Error> {
    for data in hookdata {
        Command::new(hook)
            .env("TITLE", data.title)
            .env("TITLE_FMT", data.title_fmt)
            .env("AUTHOR", data.author)
            .env("LINK", data.link)
            .env("GUID", data.guid)
            .env("PUB_DATE", data.pub_date)
            .spawn()?;
    }
    Ok(())
}
