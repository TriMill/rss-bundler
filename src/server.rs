use std::{sync::{Arc, Mutex}, thread::{self, JoinHandle}};

use reqwest::Url;
use tiny_http::Response;

use crate::State;

pub enum Void {}

pub fn start(address: &str, thread_count: usize, state: Arc<Mutex<State>>) -> Vec<JoinHandle<Void>> {
    let server = tiny_http::Server::http(address.to_owned()).unwrap();
    let server = Arc::new(server);
    let state = Arc::new(state);
    let mut threads = Vec::with_capacity(thread_count);

    for i in 0..thread_count {
        let server = server.clone();
        let state = state.clone();
        let address = address.to_owned();
        let thread = thread::spawn(move || {
            loop {
                let rq = server.recv().unwrap();
                println!("[{}] {:?}", i, rq);
                let full_url = "http://".to_string() + &address + rq.url();
                let url = match Url::parse(&full_url) { 
                    Ok(url) => url,
                    Err(e) => { 
                        let result = rq.respond(Response::from_string(e.to_string()).with_status_code(400)); 
                        if let Err(e) = result {
                            eprintln!("Error responding to request: {}", e);
                        }
                        continue 
                    }
                };
                let page = url.path().split("/").last().unwrap_or("");
                let res = match page {
                    "/rss.xml" => {
                        let guard = state.lock().unwrap();
                        let header = tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/xml"[..]).unwrap();
                        Response::from_string(&guard.rss).with_header(header)
                    },
                    "/status" => {
                        let guard = state.lock().unwrap();
                        Response::from_string(guard.status.as_ref().unwrap_or(&"Status page disabled".into()))
                    },
                    _ => Response::from_string("Not found").with_status_code(404)
                };
                let result = rq.respond(res);
                if let Err(e) = result {
                    eprintln!("Error responding to request: {}", e);
                }
            }
        });
        threads.push(thread);
    }

    threads
}