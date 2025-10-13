use log::{debug, info};
use serde::Deserialize;
use std::env::current_dir;
use std::fs;
use std::path::{Path, PathBuf};

use crate::roblox::{
    fetch_roblox_deploy_history, fetch_roblox_packages, get_roblox_version_by_git_hash,
};

// Name of the lockfile included in each Rotriever package
const LOCKFILE_NAME: &str = "lock.toml";
const INDEX_PATH: &str = "Packages/_Index";

#[derive(Deserialize, Debug)]
struct RotrieverPackageLockfile {
    dependencies: Option<Vec<String>>,
}

pub async fn install_roblox_packages(
    dest: &PathBuf,
    version: &Option<String>,
    dependencies: &Option<Vec<String>>,
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
        debug!("dependencies to keep: {:?}", dependencies);
        info!("pruning unused dependencies");

        for dependency in dependencies {
            // TODO: Handle if there are multiple versions of a package. i.e. if we
            // want to include Foundation as a dependency, we need to make sure we
            // use the right one. Maybe the most up-to-date?
            let dependency_path = dest_path.join(INDEX_PATH).join(dependency);

            let lockfile_path = dependency_path.join(LOCKFILE_NAME);
            let lockfile_content =
                fs::read_to_string(lockfile_path).expect("Failed to read lockfile");

            let lockfile: RotrieverPackageLockfile =
                toml::from_str(&lockfile_content).expect("Failed to parse TOML");

            if let Some(deps) = lockfile.dependencies {
                info!("keeping dependencies: {:?}", deps);
            }

            // TODO: Traverse over the dependencies to keep, use their lockfiles
            // to determine which _other_ dependencies to keep, and then remove
            // everything in the index and root of RobloxPackages that isn't in
            // the keep list.
        }
    }

    info!(
        "successfully installed packages from Roblox version {} to {}",
        roblox_version.git_hash,
        dest_path.display()
    );

    Ok(())
}
