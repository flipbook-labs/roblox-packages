use log::{debug, info};
use std::env::current_dir;
use std::fs;
use std::path::{Path, PathBuf};

use crate::roblox::{
    fetch_current_studio_version_id, fetch_roblox_deploy_history, fetch_roblox_packages,
    get_roblox_version_by_git_hash,
};

use crate::rotriever::prune_unused_dependencies;

pub async fn install_roblox_packages(
    dest: &PathBuf,
    version: &Option<String>,
    dependencies: &Option<Vec<String>>,
) -> Result<(), anyhow::Error> {
    let hash = if let Some(version) = version {
        debug!("looking up Roblox version {}", version);

        version.to_string()
    } else {
        debug!("no version specified, using most recent");

        fetch_current_studio_version_id().await?
    };

    let version_id = if hash.contains('.') {
        let version_history = fetch_roblox_deploy_history().await?;
        let roblox_version = get_roblox_version_by_git_hash(&hash, &version_history)
            .expect("Roblox version not found");

        roblox_version.version_id.to_string()
    } else {
        hash.to_string()
    };

    info!("downloading packages for Roblox version {}", hash);

    let mut archive = fetch_roblox_packages(&version_id).await?;

    let cwd = current_dir().unwrap();
    let dest_path = cwd.join(&dest);

    debug!("extracting packages to {:?}", &dest_path);

    // The archive uses Windows-style paths so we need to manually normalize them
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = {
            let normalized_filename = file.name().replace("\\", "/");
            let normalized_path = Path::new(&normalized_filename);
            let relative = normalized_path.strip_prefix("/").unwrap_or(normalized_path);

            dest_path.join(relative)
        };

        if file.is_dir() {
            fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            std::io::copy(&mut file, &mut outfile).unwrap();
        }
    }

    if let Some(dependencies) = dependencies {
        prune_unused_dependencies(dependencies, &dest_path);
    }

    info!(
        "successfully installed packages from Roblox version {} to {}",
        hash,
        dest_path.display()
    );

    Ok(())
}
