mod builder;

use std::collections::HashMap;
use serde_json::to_string;
use builder::{arguments, defaults::OS_TO_DOWNLOAD_HASHMAP, validate_urls};
use crate::builder::payload_builder;

struct SuppliedArgs {
    platform: String,
    language: String,
    payload: String,
    output: String,
    lhostarg: String,
    lportarg: String,
    urlarg: String,
    all: bool,
    toolchain: Option<String>, // Made optional
    default_download_url: Option<String>, // Optional until determined
}
impl SuppliedArgs {
    // Constructor
    pub fn new(argsmap: &HashMap<String, String>) -> Self {
        // Ensure that there are enough elements in the argsvec to avoid panics
        let platform = argsmap.get("platform").unwrap().to_string();
        let language = argsmap.get("language").unwrap().to_string();
        let payload = argsmap.get("payload").unwrap().to_string();
        let output = argsmap.get("output").unwrap().to_string();
        let lhostarg = argsmap.get("lhostarg").unwrap().to_string();
        let lportarg = argsmap.get("lportarg").unwrap().to_string();
        let urlarg = argsmap.get("urlarg").unwrap().to_string();
        let all = argsmap.get("all").map_or(false, |s| s.to_string().to_lowercase() == "true");
        let toolchain = argsmap.get("toolchain").map(|s| s.to_string());

        SuppliedArgs {
            platform,
            language,
            payload,
            output,
            lhostarg,
            lportarg,
            urlarg,
            all,
            toolchain,
            default_download_url: None,
        }
    }
}

#[tokio::main]
async fn main() {
    let args = arguments::getargs();
    let mut supplied_args = SuppliedArgs::new(&parse_args_to_map(args.split_whitespace().collect()));
    println!("PolyDrop");
    println!("- BYOSI (Bring-Your-Own-Script-Interpreter) Rapid Payload Deployment");
    println!(
        "OS: {}\nPayload Type: {}\nOutput: {}\n",
        supplied_args.platform, supplied_args.payload, supplied_args.output
    );
    println!(
        "LHOST: {}\nLPORT: {}\nURL: {}\n",
        supplied_args.lhostarg, supplied_args.lportarg, supplied_args.urlarg
    );
    println!("All: {}\n", supplied_args.all);
    println!("Mapping the platform real quick...\n");
    let default_download_url_map: Option<&&HashMap<&str, &str>> = OS_TO_DOWNLOAD_HASHMAP.get(supplied_args.platform.as_str());
    if supplied_args.all.to_string().contains("true") {
        all_command(&supplied_args.platform, &supplied_args.urlarg, &supplied_args.output, &supplied_args.payload, &supplied_args.lhostarg, &supplied_args.lportarg).await;
        return
    }
    match default_download_url_map {
        Some(os_map) => {
            if supplied_args.toolchain.is_none() {
                if let Some(url) = os_map.get(supplied_args.language.as_str()) {
                    supplied_args.default_download_url = Some(url.to_string());
                    println!("Toolchain is empty, selecting the default download URL.\n{}", url);
                } else {
                    println!("No default download URL found for the given language.");
                    return;
                }
            }
            if let Some(ref toolchain_url) = supplied_args.toolchain {
                match validate_urls::validate_url(toolchain_url, &supplied_args.language, &supplied_args.platform).await {
                    Ok(valid_url) if valid_url => {
                        builder::payload_builder(
                            &supplied_args.platform,
                            &supplied_args.language,
                            &supplied_args.payload,
                            &supplied_args.output,
                            &supplied_args.all.to_string(),
                            &supplied_args.lhostarg,
                            &supplied_args.lportarg,
                            &supplied_args.urlarg,
                        );
                    },
                    Ok(_) => println!("Invalid toolchain URL provided."),
                    Err(_) => println!("Error validating toolchain URL."),
                }
            } else if let Some(ref default_url) = supplied_args.default_download_url {
                match validate_urls::validate_url(default_url, &supplied_args.language, &supplied_args.platform).await {
                    Ok(valid_url) if valid_url => {
                        builder::payload_builder(
                            &supplied_args.platform,
                            &supplied_args.language,
                            &supplied_args.payload,
                            &supplied_args.output,
                            &supplied_args.all.to_string(),
                            &supplied_args.lhostarg,
                            &supplied_args.lportarg,
                            &supplied_args.urlarg,
                        );
                    },
                    Ok(_) => println!("Invalid default download URL provided"),
                    Err(e) => println!("Error validating default download URL.\n{}", e.to_string()),
                }
            }
        },
        None => {
            println!("Not a valid OS.");
            return;
        }
    }
}


fn parse_args_to_map(argsvec: Vec<&str>) -> HashMap<String, String> {
    let mut args_map = HashMap::new();
    if let Some(arg) = argsvec.get(0) {
        args_map.insert("platform".to_string(), arg.to_string());
    }
    if let Some(arg) = argsvec.get(1) {
        args_map.insert("language".to_string(), arg.to_string());
    }
    if let Some(arg) = argsvec.get(2) {
        args_map.insert("payload".to_string(), arg.to_string());
    }
    if let Some(arg) = argsvec.get(3) {
        args_map.insert("output".to_string(), arg.to_string());
    }
    if let Some(arg) = argsvec.get(4) {
        args_map.insert("all".to_string(), arg.to_string());
    }
    if let Some(arg) = argsvec.get(5) {
        args_map.insert("lhostarg".to_string(), arg.to_string());
    }
    if let Some(arg) = argsvec.get(6) {
        args_map.insert("lportarg".to_string(), arg.to_string());
    }
    if let Some(arg) = argsvec.get(7) {
        args_map.insert("urlarg".to_string(), arg.to_string());
    }
    if let Some(arg) = argsvec.get(8) {
        args_map.insert("toolchain".to_string(), arg.to_string());
    }
    args_map
}

async fn all_command(platform: &str, target_download: &str, output: &str, file_type: &str, lhost: &str, lport: &str, ){
    println!("Buckle up buttercup!");
    payload_builder(
        platform,
        "",
        file_type,
        output,
        "true",
        lhost,
        lport,
        target_download,
    );
}