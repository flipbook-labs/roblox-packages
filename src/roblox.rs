use log::debug;
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Cursor;
use zip::ZipArchive;

#[derive(Deserialize, Debug)]
pub struct RobloxVersion {
    pub version_id: String,
    pub git_hash: String,
}

pub async fn fetch_roblox_deploy_history() -> Result<Vec<RobloxVersion>, anyhow::Error> {
    let mut history = vec![];

    let res = reqwest::get("https://raw.githubusercontent.com/MaximumADHD/Roblox-Client-Tracker/refs/heads/roblox/version-history.json").await?;

    match res.status() {
        reqwest::StatusCode::OK => {
            let version_history = res.json::<HashMap<String, String>>().await?;

            for (version_id, version_hash) in version_history {
                let git_hash = version_hash
                    .strip_prefix("version-")
                    .unwrap_or(&version_hash)
                    .to_string();

                history.push(RobloxVersion {
                    version_id,
                    git_hash,
                });
            }
        }
        _ => {}
    }

    history.sort_by(|a, b| a.version_id.cmp(&b.version_id));

    Ok(history)
}

pub async fn fetch_roblox_packages(
    version: &RobloxVersion,
) -> Result<ZipArchive<Cursor<Vec<u8>>>, anyhow::Error> {
    debug!("downloading package archive for {}", version.git_hash);

    let res = reqwest::get(format!(
        "https://setup.rbxcdn.com/version-{}-extracontent-luapackages.zip",
        version.git_hash
    ))
    .await?;

    let body = res.bytes().await?;
    let cursor = Cursor::new(body.to_vec());
    let archive = ZipArchive::new(cursor).unwrap();

    Ok(archive)
}
