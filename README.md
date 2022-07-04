# RSS Bundler

RSS Bundler is a Rust program that allows you to combine multiple RSS feeds into one. An instance of RSS Bundler for the members of the [GEORGE webring](https://george.gh0.pw/) is currently hosted at https://api.trimill.xyz/george-bundle/rss.xml.

## Running

The binary takes only one argument: a path to the config file. The resulting feed is located at `/rss.xml`.

## Config

The config file is written in JSON. The following fields are required:

| Field   | Description                                                                                                                  |
|---------|------------------------------------------------------------------------------------------------------------------------------|
| `title` | The feed's title                                                                                                             |
| `link`  | A link to the feed's website                                                                                                 |
| `users` | A list of users. Each user must have a `name` field containing the user's name and a `rss` field with a link to an RSS feed. |

The following fields are optional:

| Field            | Description                                                                                    | Default value      |
|------------------|------------------------------------------------------------------------------------------------|--------------------|
| `description`    | The feed's description                                                                         | Empty              |
| `default_title`  | Title to use for posts that do not have a `title` field                                        | `<untitled>`       |
| `refresh_time`   | Minutes to wait between fetching RSS feeds                                                     | `60` (one hour)    |
| `status_page`    | Generate the status page (see below)                                                           | `true`             |
| `title_format`   | Format for post titles. Use `{name}` for the user's name and `{title}` for the original title. | `[{name}] {title}` |
| `worker_threads` | Number of threads to spawn for the web server.                                                 | `4`                |
| `port`           | Port number for web server                                                                     | `4400`             |
| `host`           | Host for web server                                                                            | `127.0.0.1`        |

Here is an example configuration:

```json
{
    "title": "Example bundle",
    "description": "Demonstration of RSS Bundler",
    "link": "https://github.com/trimill/rss-bundler",
    "refresh_time": 30,
    "title_format": "({name}) {title}",
    "worker_threads": 2,
    "port": 5000,
    "users": [
        { "name": "trimill", "rss": "https://trimill.xyz/blog/rss.xml" }
    ]
}
```

## Status page

RSS Bundler also generates a status page, available at `/status`. This page shows the last date a feed was fetched and parsed successfully and, if the last try was erroneous, the error that occured. If an error occurs while fetching or parsing a feed, the last good version will be used instead.
