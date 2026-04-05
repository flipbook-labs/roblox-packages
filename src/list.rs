use std::collections::HashSet;

use crate::roblox::fetch_roblox_deploy_history;

pub async fn list_roblox_versions(limit: &usize) -> Result<(), anyhow::Error> {
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
            "{} ({})",
            roblox_version.version_id, roblox_version.git_hash
        )
    }

    Ok(())
}
