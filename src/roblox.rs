use anyhow::{Context, anyhow};
use log::debug;
use regex::Regex;
use std::io::Cursor;
use zip::ZipArchive;

#[derive(Debug)]
pub struct RobloxVersion {
    pub version_id: String,
    pub git_hash: String,
    pub timestamp: String,
}

pub async fn fetch_roblox_deploy_history() -> Result<Vec<RobloxVersion>, reqwest::Error> {
    // New Studio64 version-ffd2994ae3bd41ab at 9/9/2025 4:21:09 PM, file version: 0, 690, 0, 6900721, git hash: 0.690.0.6900721 ...
    let re = Regex::new(
        r#"New (?P<target>\w+) version-(?P<version_id>[\w\d]+) at (?P<timestamp>.+), file version: .+, git hash: (?P<git_hash>[\d\.]+)"#
    ).unwrap();

    let mut history = vec![];

    let res = reqwest::get("https://setup.rbxcdn.com/DeployHistory.txt").await?;

    match res.status() {
        reqwest::StatusCode::OK => {
            let content = res.text().await?;

            for (_, [target, version_id, timestamp, git_hash]) in
                re.captures_iter(&content).map(|c| c.extract())
            {
                if target != "Studio64" {
                    continue;
                }

                if version_id == "hidden" {
                    continue;
                }

                history.push(RobloxVersion {
                    version_id: version_id.to_string(),
                    git_hash: git_hash.to_string(),
                    timestamp: timestamp.to_string(),
                })
            }
        }
        _ => {}
    }

    Ok(history)
}

pub async fn fetch_current_studio_version_id() -> Result<String, anyhow::Error> {
    let res = reqwest::get("https://setup.rbxcdn.com/mac/versionStudio").await?;
    let body = res.error_for_status()?.text().await?;
    let trimmed = body.trim();

    if let Some(version_id) = trimmed.strip_prefix("version-") {
        Ok(version_id.to_string())
    } else {
        Err(anyhow!(
            "unexpected response from versionQTStudio endpoint: {}",
            trimmed
        ))
    }
}

pub async fn fetch_roblox_packages(
    version_id: &str,
) -> Result<ZipArchive<Cursor<Vec<u8>>>, anyhow::Error> {
    debug!("downloading package archive for {}", version_id);

    let res = reqwest::get(format!(
        "https://setup.rbxcdn.com/version-{}-extracontent-luapackages.zip",
        version_id
    ))
    .await?;

    let body = res.error_for_status()?.bytes().await?;
    let cursor = Cursor::new(body.to_vec());
    let archive =
        ZipArchive::new(cursor).context("downloaded package archive is not a valid zip archive")?;

    Ok(archive)
}

pub fn get_roblox_version_by_git_hash<'a>(
    git_hash: &str,
    version_history: &'a [RobloxVersion],
) -> Option<&'a RobloxVersion> {
    for version in version_history.iter().rev() {
        if version.git_hash == git_hash {
            return Some(version);
        }
    }

    None
}
