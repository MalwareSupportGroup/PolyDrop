use reqwest::blocking::get;
use reqwest::{Error};
use scraper::{Html, Selector};
use regex::Regex;
use std::collections::HashSet;

/// Recursively fetch and list contents of directories that match the version pattern.
pub fn recurse_directory(
    base_url: &str,
    version_pattern: &str,
    visited: &mut HashSet<String>,
) -> Result<(), Error> {
    let use_pattern = if version_pattern.is_empty() {
        r"^\d+(\.\d+)*?/?$"
    } else {
        version_pattern
    };
    let version_regex = Regex::new(use_pattern).unwrap();

    inner_recurse_directory(base_url, &version_regex, visited)?;

    Ok(())
}

fn inner_recurse_directory(
    url: &str,
    version_regex: &Regex,
    visited: &mut HashSet<String>,
) -> Result<(), Error> {
    if visited.contains(url) {
        return Ok(());
    }

    visited.insert(url.to_string());

    let response = get(url)?.text()?;
    let document = Html::parse_document(&response);
    let selector = Selector::parse("a").unwrap();

    for element in document.select(&selector) {
        if let Some(link) = element.value().attr("href") {
            let next_url = format!("{}{}", url, link);

            if is_valid_directory(&link, version_regex) {
                println!("Directory: {}", link);
                inner_recurse_directory(&next_url, version_regex, visited)?;
            } else if !link.contains("://") && !link.contains("#") && !link.contains("?") {
                println!("File: {}", link);
            }
        }
    }

    Ok(())
}

fn is_valid_directory(link: &str, version_regex: &Regex) -> bool {
    version_regex.is_match(link) && link.ends_with('/')
}