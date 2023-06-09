use std::collections::HashMap;
use std::{env, io};
use std::future::Future;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::ops::DerefMut;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use anyhow::anyhow;
use cached::proc_macro::once;

use chrono::{DateTime, LocalResult, TimeZone, Utc};
use reqwest::{Client};
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Error;
use log::{debug, error, LevelFilter};
use prometheus_client::registry::Registry;
use lazy_static::lazy_static;
use prometheus_client::encoding::text::encode;
use tokio::signal::unix::{signal, SignalKind};
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};

use data::*;
use crate::metrics::create_metrics;

mod data;
mod metrics;

lazy_static! {
    static ref ORGANIZATION: String = env::var("ORG").expect("No organization provided for environment variable 'ORG'!");
    static ref HEADERS: HeaderMap = create_default_headers(env::var("TOKEN").expect("No github-token provided for environment variable 'TOKEN'!")).expect("");
    static ref PORT: u16 = env::var("PORT").expect("No port provided for environment variable 'PORT'!").parse::<u16>().expect("Not parsable to u16");
    static ref LAST_SCRAPE: Mutex<DateTime<Utc>> = Mutex::new({
        let last = Utc.with_ymd_and_hms(2007, 1, 1, 1, 1, 1);
    let last = match last {
        LocalResult::None => { panic!("No timestamp") }
        LocalResult::Single(dt) => { dt }
        LocalResult::Ambiguous(_, _) => { panic!("Ambiguous timestamp") }
    };
        last
    });
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logging()?;
    let mut registry = <Registry>::default();
    create_metrics(&mut registry);
    let metrics_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), *PORT);
    start_metrics_server(metrics_addr, registry).await;
    Ok(())
}

/// Start a HTTP server to report metrics.
pub async fn start_metrics_server(metrics_addr: SocketAddr, registry: Registry) {
    let mut shutdown_stream = signal(SignalKind::terminate()).unwrap();

    eprintln!("Starting metrics server on {metrics_addr}");

    let registry = Arc::new(registry);
    Server::bind(&metrics_addr)
        .serve(make_service_fn(move |_conn| {
            let registry = registry.clone();
            async move {
                let handler = make_handler(registry);
                Ok::<_, io::Error>(service_fn(handler))
            }
        }))
        .with_graceful_shutdown(async move {
            shutdown_stream.recv().await;
        })
        .await
        .unwrap();
}

/// This function returns a HTTP handler (i.e. another function)
pub fn make_handler(
    registry: Arc<Registry>,
) -> impl Fn(Request<Body>) -> Pin<Box<dyn Future<Output = io::Result<Response<Body>>> + Send>> {
    // This closure accepts a request and responds with the OpenMetrics encoding of our metrics.
    move |_req: Request<Body>| {
        let reg = registry.clone();
        Box::pin(async move {
            let mut buf = String::new();
            encode(&mut buf, &reg.clone())
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
                .map(|_| {
                    let body = Body::from(buf);
                    Response::builder()
                        .header(
                            hyper::header::CONTENT_TYPE,
                            "application/openmetrics-text; version=1.0.0; charset=utf-8",
                        )
                        .body(body)
                        .unwrap()
                })
        })
    }
}

fn now() -> DateTime<Utc> {
    DateTime::from(SystemTime::now())
}

#[once(time = 10, option = true, sync_writes = true)]
pub async fn get_all_commits_since_last_and_update_timestamp() -> Option<RepositoriesWithCommits> {
    let client = Client::new();
    let now: DateTime<Utc> = DateTime::from(SystemTime::now()); //for whatever reason calling now() would result in a compiler error due to the used macro
    let last_scrape = match LAST_SCRAPE.lock() {
        Ok(guard) => { (*guard).into() }
        Err(_) => {
            error!("Failed to acquire mutex gard of last scrape!");
            return None;
        }
    };
    let result = get_all_commits_since(&client, &HEADERS, ORGANIZATION.as_str(), last_scrape).await;
    let data = match result {
        Ok(value) => { Some(RepositoriesWithCommits { data: value }) }
        Err(error) => {
            error!("Some error occurred during fetching of data from github {error}");
            None
        }
    };
    let mut last_scrape_ref = LAST_SCRAPE.lock().unwrap();
    *last_scrape_ref.deref_mut() = now;
    data
}

async fn get_all_commits_since(client: &Client, headers: &HeaderMap, organization: &str, since: DateTime<Utc>) -> anyhow::Result<Vec<RepositoryAndCommits>> {
    let repositories = list_organization_repositories(&client, headers.clone(), organization).await?;
    let mut data = Vec::new();
    for repository in repositories {
        debug!("Fetching commits for {repo}...", repo=&repository.name);
        let commits = list_commits_in_repository_since(&client, repository.full_name.clone(), headers.clone(), since).await?;
        let mut full_data = Vec::new();
        for commit in commits {
            debug!("Fetching details for {commit}...", commit=&commit.commit.message);
            let full_commit_data = get_full_commit_data(&client, headers.clone(), &repository.full_name.clone(), commit).await?;
            full_data.push(full_commit_data);
        }
        let repository_and_commits = RepositoryAndCommits::from(repository, full_data);
        data.push(repository_and_commits);
    }
    Ok(data)
}

async fn get_full_commit_data(client: &Client, headers: HeaderMap, full_repository_name: &str, commit: Commit) -> anyhow::Result<FullCommitData> {
    let details = fetch_commit(&client, headers.clone(), full_repository_name, commit.sha.as_str()).await?;
    Ok(FullCommitData::from(commit, details))
}

async fn list_organization_repositories(client: &Client, headers: HeaderMap, organization: &str) -> anyhow::Result<Vec<MinimalRepository>> {
    let url = format!("https://api.github.com/orgs/{organization}/repos", organization = organization);
    let response = client
        .get(url)
        .headers(headers)
        .send()
        .await?;
    let status_code = get_status_code(&response);
    debug!("Retrieving all repositories of {org} - Status code: {code}", org = organization, code = status_code);
    let json_string = response.text().await?;
    let conversion_result: Result<Vec<MinimalRepository>, _> = serde_json::from_str(&json_string);
    let repositories = handle_json_conversion(status_code, json_string, conversion_result);
    Ok(repositories)
}

fn get_status_code(response: &reqwest::Response) -> u16 {
    let status_code = &response.status().clone();
    let status_code = status_code.as_u16();
    status_code
}

async fn list_commits_in_repository_since(client: &Client, full_repository_name: String, headers: HeaderMap, since: DateTime<Utc>) -> anyhow::Result<Vec<Commit>> {
    let mut params = HashMap::new();
    params.insert("since", since.to_string());
    let url = format!("https://api.github.com/repos/{full_name}/commits", full_name = full_repository_name);
    let response = client.get(url)
        .query(&params)
        .headers(headers)
        .send()
        .await?;
    let status_code = get_status_code(&response);
    debug!("Retrieving all commits of {repo} - Status code: {code}", repo = full_repository_name, code = status_code);
    let json_string = response.text().await?;
    let conversion_result: Result<Vec<Commit>, _> = serde_json::from_str(&json_string);
    let commits = handle_json_conversion(status_code, json_string, conversion_result);
    Ok(commits)
}

async fn fetch_commit(client: &Client, headers: HeaderMap, full_repository_name: &str, commit_reference: &str) -> anyhow::Result<CommitChangeDetails> {
    let url = format!("https://api.github.com/repos/{full_name}/commits/{reference}", full_name = full_repository_name, reference = commit_reference);
    let response = client.get(url)
        .headers(headers)
        .send()
        .await?;
    let status_code = get_status_code(&response);
    debug!("Retrieving all details of commit {commit} - Status code: {code}", commit = commit_reference, code = status_code);
    let json_string = response.text().await?;
    let conversion_result: Result<CommitChangeDetails, _> = serde_json::from_str(&json_string);
    let commit_details = match conversion_result {
        Ok(details) => { details }
        Err(e) => {
            error!("Failed to deserialize JSON data: {error}", error = e);
            error!("JSON data: {json}", json = json_string);
            error!("{}", status_code);
            return Err(anyhow!(e));
        }
    };
    Ok(commit_details)
}

fn handle_json_conversion<Type>(status_code: u16, json_string: String, conversion_result: Result<Vec<Type>, Error>) -> Vec<Type> {
    match conversion_result {
        Ok(conversions) => conversions,
        Err(e) => {
            error!("Failed to deserialize JSON data: {error}", error = e);
            error!("JSON data: {json}", json = json_string);
            error!("{}", status_code);
            Vec::new()
        }
    }
}

fn create_default_headers(token: String) -> anyhow::Result<HeaderMap> {
    let mut headers = HeaderMap::new();
    headers.insert("Accept", HeaderValue::from_static("application/vnd.github+json"));
    headers.insert("User-Agent", HeaderValue::from_static("github-exporter-arm64-rs"));
    headers.insert("Authorization", HeaderValue::from_str(format!("Bearer {token}").as_str())?);
    Ok(headers)
}

fn init_logging() -> anyhow::Result<()> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} - {} - {}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Debug)
        //.chain(io::stderr())
        .chain(fern::log_file("logs/log.log")?)
        .apply()?;
    Ok(())
}
