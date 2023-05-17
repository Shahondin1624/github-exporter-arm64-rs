use std::collections::HashMap;
use std::{env, io};
use anyhow::anyhow;

use chrono::{DateTime, LocalResult, TimeZone, Utc};
use reqwest::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Error;
use log::{debug, error, info, LevelFilter};

use data::*;

mod data;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logging()?;
    let token = env::var("TOKEN").ok();
    if token.is_some() {
        info!("Retrieved api-token")
    } else {
        info!("Failed to retrieve api-token")
    }
    let client = Client::new();

    let headers = create_default_headers(token)?;

    let repositories = list_organization_repositories(&client, headers.clone(), "KlaraOppenheimerSchule").await?;

    let date_time = Utc.with_ymd_and_hms(2015, 11, 28, 21, 0, 9);
    let date_time = match date_time {
        LocalResult::None => { panic!("No timestamp") }
        LocalResult::Single(dt) => { dt }
        LocalResult::Ambiguous(_, _) => { panic!("Ambiguous timestamp") }
    };

    for repository in repositories {
        println!("{}", repository.name);
        let commits = list_commits_in_repository_since(&client, repository.full_name.clone(), headers.clone(), date_time).await?;
        for commit in commits {
            let details = fetch_commit(&client, headers.clone(), &repository.full_name.clone(), commit.sha.as_str()).await?;
            println!("{:?}", details);
        }
    }

    Ok(())
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

fn get_status_code(response: &Response) -> u16 {
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

fn create_default_headers(token: Option<String>) -> anyhow::Result<HeaderMap> {
    let mut headers = HeaderMap::new();
    headers.insert("Accept", HeaderValue::from_static("application/vnd.github+json"));
    headers.insert("User-Agent", HeaderValue::from_static("github-exporter-arm64-rs"));
    match token {
        None => {}
        Some(value) => {
            headers.insert("Authorization", HeaderValue::from_str(format!("Bearer {token}", token = value).as_str())?);
        }
    }
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
        .level(LevelFilter::Info)
        //.chain(io::stderr())
        .chain(fern::log_file("logs/log.log")?)
        .apply()?;
    Ok(())
}
