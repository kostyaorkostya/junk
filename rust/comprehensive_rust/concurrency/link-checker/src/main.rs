use reqwest::blocking::Client;
use reqwest::Url;
use scraper::{Html, Selector};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("bad http response: {0}")]
    BadResponse(String),
}

#[derive(Debug)]
struct CrawlCommand {
    url: Url,
    extract_links: bool,
}

fn visit_page(client: &Client, command: &CrawlCommand) -> Result<Vec<Url>, Error> {
    println!("Checking {:#}", command.url);
    let response = client.get(command.url.clone()).send()?;
    if !response.status().is_success() {
        return Err(Error::BadResponse(response.status().to_string()));
    }

    let mut link_urls = Vec::new();
    if !command.extract_links {
        return Ok(link_urls);
    }

    let base_url = response.url().to_owned();
    let body_text = response.text()?;
    let document = Html::parse_document(&body_text);

    let selector = Selector::parse("a").unwrap();
    let href_values = document
        .select(&selector)
        .filter_map(|element| element.value().attr("href"));
    for href in href_values {
        match base_url.join(href) {
            Ok(link_url) => {
                link_urls.push(link_url);
            }
            Err(err) => {
                println!("On {base_url:#}: ignored unparsable {href:?}: {err}");
            }
        }
    }
    Ok(link_urls)
}

struct SingleDomainCrawlerState {
    domain: String,
    visited: std::collections::HashSet<Url>,
}

impl SingleDomainCrawlerState {
    fn new(seed: &Url) -> Self {
        let domain = seed.domain().unwrap().to_string();
        let visited = std::collections::HashSet::new();
        Self { domain, visited }
    }

    fn should_extract_links(&self, url: &Url) -> bool {
        match url.domain() {
            None => false,
            Some(domain) => self.domain == domain,
        }
    }

    fn mark_visited(&mut self, url: &Url) -> bool {
        self.visited.insert(url.clone())
    }
}

type CrawlResult = Result<Vec<Url>, (Url, Error)>;
fn spawn_threads(
    commands: mpsc::Receiver<CrawlCommand>,
    results: mpsc::Sender<CrawlResult>,
    thread_count: u32,
) -> () {
    let commands = Arc::new(Mutex::new(commands));
    for _ in 0..thread_count {
        let commands = commands.clone();
        let results = results.clone();
        thread::spawn(move || {
            let client = Client::new();
            loop {
                let Ok(command) = commands.lock().unwrap().recv() else {
                    break;
                };
                let result = visit_page(&client, &command).map_err(|err| (command.url, err));
                let Ok(()) = results.send(result) else {
                    break;
                };
            }
        });
    }
}

fn event_loop(
    seed: &Url,
    commands: mpsc::Sender<CrawlCommand>,
    results: mpsc::Receiver<CrawlResult>,
) -> Vec<Url> {
    let mut state = SingleDomainCrawlerState::new(&seed);
    commands
        .send(CrawlCommand {
            url: seed.clone(),
            extract_links: true,
        })
        .unwrap();
    let mut pending = 1;
    let mut bad_urls = Vec::new();
    while pending > 0 {
        let result = results.recv().unwrap();
        pending -= 1;
        match result {
            Ok(links) => {
                for url in links {
                    if state.mark_visited(&url) {
                        let extract_links = state.should_extract_links(&url);
                        commands.send(CrawlCommand { url, extract_links }).unwrap();
                        pending += 1
                    }
                }
            }
            Err((url, err)) => {
                bad_urls.push(url);
                println!("Got crawling error: {:#}", err);
                continue;
            }
        }
    }
    bad_urls
}

fn check_links(seed: &Url, thread_count: u32) -> Vec<Url> {
    let (results_sender, results_receiver) = mpsc::channel::<CrawlResult>();
    let (commands_sender, commands_receiver) = mpsc::channel::<CrawlCommand>();
    spawn_threads(commands_receiver, results_sender, thread_count);
    event_loop(seed, commands_sender, results_receiver)
}

fn main() -> anyhow::Result<()> {
    let seed = reqwest::Url::parse("https://www.google.org")?;
    let bad_urls = check_links(&seed, 32);
    println!("Bad URLs: {:#?}", bad_urls);
    Ok(())
}
