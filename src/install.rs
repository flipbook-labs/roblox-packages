use log::{debug, info};
use std::fs;
use std::path::{Path, PathBuf};
use std::{env::current_dir, fs::remove_dir_all};

use crate::roblox::{
    fetch_roblox_deploy_history, fetch_roblox_packages, get_roblox_version_by_git_hash,
};

pub async fn install_roblox_packages(
    dest: &PathBuf,
    version: &Option<String>,
) -> Result<(), reqwest::Error> {
    let version_history = fetch_roblox_deploy_history().await?;

    let roblox_version = if let Some(version) = version {
        debug!("looking up Roblox version {}", version);
        get_roblox_version_by_git_hash(version, &version_history).expect("Roblox version not found")
    } else {
        debug!("no version specified, using most recent");
        version_history
            .last()
            .expect("could not get a most recent version")
    };

    info!(
        "downloading packages for Roblox version {}",
        roblox_version.git_hash
    );

    let mut archive = fetch_roblox_packages(&roblox_version).await?;

    let cwd = current_dir().unwrap();
    let dest_path = cwd.join(&dest);

    debug!("extracting packages to {:?}", &dest_path);

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

    info!(
        "successfully extracted packages from Roblox version {} to {}",
        roblox_version.git_hash,
        dest_path.display()
    );

    // if roblox_studio_packages_path.exists() {
    //     if dest.exists() {
    //         info!("removing existing destination {:?}...", dest_path);
    //         remove_dir_all(&dest_path).unwrap();
    //     }

    //     info!("copying packages from {:?} ", roblox_studio_packages_path);
    //     copy_dir(roblox_studio_packages_path, &dest_path).unwrap();

    //     info!("successfully installed Roblox packages to {:?}", dest_path);
    // }

    Ok(())
}
