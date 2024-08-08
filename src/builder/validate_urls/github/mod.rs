use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub user_type: String,
    pub site_admin: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Asset {
    pub url: String,
    pub id: u64,
    pub node_id: String,
    pub name: String,
    pub label: String,
    pub uploader: Author,
    pub content_type: String,
    pub state: String,
    pub size: u64,
    pub download_count: u64,
    pub created_at: String,
    pub updated_at: String,
    pub browser_download_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Release {
    pub url: String,
    pub assets_url: String,
    pub upload_url: String,
    pub html_url: String,
    pub id: u64,
    pub author: Author,
    pub node_id: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: String,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: String,
    pub assets: Vec<Asset>,
}

pub fn extract_browser_download_urls(json_data: &str) -> Vec<String> {
    let release: Release = serde_json::from_str(json_data).expect("Failed to parse JSON");
    release
        .assets
        .iter()
        .map(|asset| asset.browser_download_url.clone())
        .collect()
}
