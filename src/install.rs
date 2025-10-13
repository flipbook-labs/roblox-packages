use log::{debug, info};
use serde::Deserialize;
use std::env::current_dir;
use std::path::{Path, PathBuf};
use std::{fs, vec};

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

fn prune_unused_dependencies(dependencies: &Vec<String>, packages_path: &Path) {
    info!("pruning unused dependencies");

    info!("root dependencies to keep: {:?}", dependencies);

    fn process_dependency(
        dependency: &str,
        packages_path: &Path,
        dependencies_to_keep: &mut Vec<String>,
    ) {
        info!("keeping dependency: {}", dependency);

        // TODO: Handle if there are multiple versions of a package. i.e. if we
        // want to include Foundation as a dependency, we need to make sure we
        // use the right one. Maybe the most up-to-date?

        let dependency_path = packages_path.join(INDEX_PATH).join(dependency);

        let lockfile_path = dependency_path.join(LOCKFILE_NAME);
        let lockfile_content = fs::read_to_string(lockfile_path).expect("Failed to read lockfile");

        let lockfile: RotrieverPackageLockfile =
            toml::from_str(&lockfile_content).expect("Failed to parse TOML");

        // TODO: Traverse over the dependencies to keep, use their lockfiles
        // to determine which _other_ dependencies to keep, and then remove
        // everything in the index and root of RobloxPackages that isn't in
        // the keep list.

        if let Some(deps) = lockfile.dependencies {
            for sub_dependency in &deps {
                let sub_dependency_parts: Vec<&str> = sub_dependency.split(" ").collect();
                let sub_dependency_name = sub_dependency_parts[1];

                info!("keeping dependency: {}", sub_dependency_name);

                dependencies_to_keep.push(sub_dependency_name.to_string());

                process_dependency(sub_dependency_name, packages_path, dependencies_to_keep);

                // TODO: Go up into the index and process the sub-dependency

                // TODO: Traverse over the dependencies of these
                // dependencies and so on until we have the full picture
            }
        }
    }

    let mut dependencies_to_keep: Vec<String> = vec![];

    for dependency in dependencies {
        process_dependency(dependency, packages_path, &mut dependencies_to_keep);
    }

    info!("dependencies to keep: {:?}", dependencies_to_keep);
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
        prune_unused_dependencies(dependencies, &dest_path);
    }

    info!(
        "successfully installed packages from Roblox version {} to {}",
        roblox_version.git_hash,
        dest_path.display()
    );

    Ok(())
}
