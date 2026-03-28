use anyhow::Result;
use std::collections::HashSet;

use crate::roblox::{fetch_current_studio_version_id, fetch_roblox_deploy_history};

pub async fn list_historic_roblox_versions(limit: &usize) -> Result<()> {
    let version_history = fetch_roblox_deploy_history().await?;
    let mut seen_hashes = HashSet::new();
    let mut deduped_versions = Vec::new();

    for roblox_version in version_history.iter().rev() {
        if !seen_hashes.insert(&roblox_version.git_hash) {
            continue;
        }

        deduped_versions.push(roblox_version);

        if deduped_versions.len() == *limit {
            break;
        }
    }

    for roblox_version in deduped_versions.into_iter().rev() {
        println!(
            "{} ({}) - {}",
            roblox_version.git_hash, roblox_version.version_id, roblox_version.timestamp
        )
    }

    Ok(())
}

pub async fn list_current_roblox_version() -> Result<()> {
    let version = fetch_current_studio_version_id().await?;

    println!("{}", version);

    Ok(())
}
