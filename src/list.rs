use crate::roblox::fetch_roblox_deploy_history;

pub async fn list_roblox_versions(limit: &usize) -> Result<(), reqwest::Error> {
    let version_history = fetch_roblox_deploy_history().await?;

    for roblox_version in version_history.iter().rev().take(*limit).rev() {
        println!("{} - {}", roblox_version.git_hash, roblox_version.timestamp)
    }

    Ok(())
}
