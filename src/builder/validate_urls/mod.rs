use anyhow::{Context, Error, Result};
use chrono::Datelike;
use julia::*;
use kuchiki::parse_html;
use kuchiki::traits::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use reqwest::{Client, Response};
use std::collections::HashMap;
use std::convert::TryInto;
use std::default::Default;
use std::time::{SystemTime, UNIX_EPOCH};
use colored::Colorize;
use python::recurse_directory;
use url::Url;
use ext_to_os_payload_map::*;
mod constants;
mod github;
mod julia;
mod ext_to_os_payload_map;
mod python;

// no longer needed.
const DLANG_CSS_SELECTOR: &[&str] = &["#content"];
const PYTHON_CSS_SELECTOR: &[&str] = &[""];
const USER_AGENTS: &[&str] = &[
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.212 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.128 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/88.0.4324.190 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.141 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 11.6; rv:92.0) Gecko/20100101 Firefox/92.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.3",
];

// will remove this, as its no longer needed.
// fn get_downloads() -> HashMap<&'static str, &'static str> {
//     let mut download_links: HashMap<&str, &str> = HashMap::new();
//     download_links.insert("mac-tcl", "https://tclkits.rkeene.org/fossil/raw/"); // added
//     download_links.insert(
//         "base-tcl",
//         "https://storage.googleapis.com/google-code-archive-downloads/v2/code.google.com/tclkit/",
//     ); // added
//     download_links.insert(
//         "dart",
//         "https://storage.googleapis.com/dart-archive/channels/main/raw/latest/sdk/",
//     );
//     download_links.insert("nodejs", "https://nodejs.org/en/download/"); // added
//     download_links.insert(
//         "fsharp",
//         "https://download.visualstudio.microsoft.com/",
//     ); // added
//     download_links
// }

// ^ I DO NOT HAVE ANY CSS/XPATH FOR THOSE, some of those explcitily require a browser. work in progress.
// @todo
pub(crate) fn get_dir_listings() -> HashMap<&'static str, &'static str> {
    let mut listings = HashMap::new();

    listings.insert("python", "https://www.python.org/ftp/");
    listings.insert("dlang", "https://downloads.dlang.org/releases/");
    listings.insert("win_php", "https://windows.php.net/downloads/releases/");
    listings.insert("gnu_php", "https://dl.static-php.dev/static-php-cli/common/");
    listings.insert("julia", "https://julialang-s3.julialang.org/");
    listings.insert("golang", "https://go.dev/dl/");

    listings
}


// Validate if the user-supplied URL is a valid archive link and present on the website
pub async fn validate_url(user_url: &str, language: &str, platform: &str) -> Result<bool> {
    let supplied_url = Url::parse(user_url).context("Invalid user-supplied URL")?;
    let safe_lang = if platform.contains("windows") && language.contains("php") {
        "win_php"
    } else if (platform.contains("linux") ||
        platform.contains("mac") && language.contains("php")) {
        "gnu_php"
    } else {
        language
    };
    let selected_dir_listing = get_dir_listings().get(safe_lang).map(|s| s.to_string()).unwrap_or("None".to_string());
    let response = perform_get_request(&user_url).await;
    match response {
        Ok(response) => {
            if response.status().is_success() {
                // let html = fetch_html(user_url).await?;
                // let archive_links = extract_archive_links(&html, &selected_dir_listing);
                if (supplied_url.path().ends_with(".zip") || supplied_url.path().ends_with(".tar.gz"))
                {
                    Ok(true)
                } else {
                    println!("Alternative archives:");
                    for i in fetch_and_list_archives(&selected_dir_listing).await.unwrap() {
                        if i.contains(platform) {
                            println!("{}", i.to_string());
                        }
                    }
                    Ok(false)
                }
            } else {
                Ok(false)
            }
        },
        _ => {
            println!("Alternate archives to use:");
            for i in fetch_and_list_archives(&selected_dir_listing).await.unwrap() {
                if i.contains(platform) {
                    println!("{}", i.to_string());
                }
            }
            Err(anyhow::anyhow!("Failed to parse the url, please double check the supplied URL to the list returned."))
        }
    }

}

    // Fetch the content of the download page and list all zip and tar.gz archives
async fn fetch_and_list_archives(url: &str) -> Result<Vec<String>> {
    let dir_listings = get_dir_listings();
    let supplied_url = Url::parse(url).context("Invalid user-supplied URL")?;
    let base_url = supplied_url.origin().ascii_serialization();
    if let Some(&anticipated_url) = dir_listings
        .values()
        .find(|&&u| u.contains(base_url.as_str()))
    {
        if base_url.contains("github.com") {
            println!("It's GitHub, going to try and parse with CSS selectors.");
            let html = fetch_html(anticipated_url).await?;
            let archive_links: Vec<String> = github::extract_browser_download_urls(&html);
            let filtered_links: Vec<String> = archive_links
                .into_iter()
                .filter(|link| {
                    link.contains("crystal")
                        || link.contains("bun")
                        || link.contains("deno")
                        || link.contains("vlang")
                })
                .collect();
            for link in &filtered_links {
                let anticipated_url = if link.contains("crystal") {
                    constants::CRYSTAL_LANG_API[0]
                } else if link.contains("vlang") {
                    constants::VLANG_API[0]
                } else if link.contains("bun") {
                    constants::BUN_API[0]
                } else if link.contains("deno") {
                    constants::DENO_API[0]
                } else {
                    continue;
                };
            }
            let github_json = fetch_html(anticipated_url).await?;
            let archive_links = github::extract_browser_download_urls(&github_json);
            println!("Github releases for {}", anticipated_url);
            Ok(archive_links)
        } else if base_url.contains("python.org") {
            let html = fetch_html(anticipated_url).await?;
            // let archive_links = recurse_directory("https://www.python.org/ftp/python/", "", &mut Default::default()).await?;
            let archive_links = find_href_values_github(&html, PYTHON_CSS_SELECTOR[0], &anticipated_url);
            Ok(archive_links)
        } else if base_url.contains("downloads.dlang.org") {
            // Format DLang URL with current year
            let dlang_url = format!(
                "https://downloads.dlang.org/releases/{}/",
                get_current_year()
            );
            println!("Using: {}", dlang_url);
            // Fetch archived HTML again since it's a different URL
            let dlang_html = fetch_html(&dlang_url).await?;
            let archive_links =
                find_href_values_github(&dlang_html, DLANG_CSS_SELECTOR[0], &dlang_url);
            Ok(archive_links)
        } else if base_url.contains("julialang-s3.julialang.org") {
            let j_url = dir_listings["julia"];
            let julia_xml = fetch_html(j_url).await?;
            let archive_links = parse_julia_s3_data(&julia_xml, &j_url);
            Ok(archive_links)
        } else {
            let html = fetch_html(anticipated_url).await?;
            let archive_links = extract_archive_links(&html, anticipated_url);
            Ok(archive_links)
        }
    } else {
        Err(anyhow::anyhow!("Base URL not found in the download list"))
    }
}
// Perform a GET request
async fn perform_get_request(url: &str) -> Result<Response> {
    let user_agent = USER_AGENTS
        .choose(&mut thread_rng())
        .expect("No User-Agent strings available");
    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::limited(3))
        .build()?;
    let response = client
        .get(url)
        .header("User-Agent", *user_agent)
        .send()
        .await
        .with_context(|| format!("Failed to fetch URL: {}", url))?;
    Ok(response)
}

// Fetch the HTML content of the given URL
async fn fetch_html(url: &str) -> Result<String> {
    let response = perform_get_request(url).await?;
    if response.status().is_success() {
        let body = response.text().await?;
        Ok(body)
    } else {
        let status = response.status();
        let error_text = response.text().await?;
        println!("Failed to fetch: {:?}", error_text);
        Err(anyhow::anyhow!("Failed to fetch URL: {}", status))
    }
}

// Extract all zip and tar.gz links from the HTML content
fn extract_archive_links(html: &str, base_url: &str) -> Vec<String> {
    let mut archive_links = Vec::new();
    let base = Url::parse(base_url).expect("Base URL is invalid");
    let document = parse_html().one(html);
    for css_match in document.select("a").unwrap() {
        let attributes = css_match.attributes.borrow();
        if let Some(href) = attributes.get("href") {
            if (href.ends_with(".zip") || href.ends_with(".tar.gz"))
                && !href.contains("debug")
                && !href.contains("devel")
                && !href.contains("test")
                && !href.contains("src")
            {
                if let Ok(url) = base.join(href) {
                    archive_links.push(url.to_string());
                }
            }
        }
    }
    archive_links
}

fn find_href_values_github(html: &str, css_selector: &str, url: &str) -> Vec<String> {
    // Parse HTML
    let document = parse_html().one(html);
    // Query document using CSS selector
    let mut hrefs = Vec::new();
    for node in document
        .select(css_selector)
        .expect("Failed to locate Table")
    {
        for a_tag in node
            .as_node()
            .select("a")
            .expect("Failed to select anchor tags")
        {
            if let Some(href) = a_tag.attributes.borrow().get("href") {
                // Filter and push valid href attributes
                if (href.ends_with(".zip") || href.ends_with(".tar.gz"))
                    && !href.contains("src")
                    && !href.contains("devel")
                    && !href.contains("test")
                    && !href.contains("debug")
                {
                    let full_url = format!("{}{}", url, &href.to_string()[2..]);
                    hrefs.push(full_url);
                }
            }
        }
    }

    hrefs
}

fn get_current_year() -> i32 {
    // Get the current system time
    let now = SystemTime::now();
    // Convert to duration since UNIX_EPOCH
    let duration_since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    // Get the number of seconds since UNIX_EPOCH
    let seconds_since_epoch = duration_since_epoch.as_secs();
    // Convert to `chrono::NaiveDateTime` for easier date manipulation
    let naive_datetime =
        chrono::NaiveDateTime::from_timestamp(seconds_since_epoch.try_into().unwrap(), 0);
    // Get the current date from the NaiveDateTime
    let date = naive_datetime.date();
    // Extract the year
    date.year()
}
