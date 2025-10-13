use std::{
    fs,
    path::{Path, PathBuf},
};

use glob::glob;
use log::{debug, info};
use serde::Deserialize;

// Collection of functions for working with Rotriever's package index

// Name of the lockfile included in each Rotriever package
const LOCKFILE_NAME: &str = "rotriever.lock";

#[derive(Deserialize, Debug, Clone)]
struct RotrieverLockfilePackage {
    name: String,
    version: String,
    dependencies: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Clone)]
struct RotrieverLockfile {
    package: Vec<RotrieverLockfilePackage>,
}

fn get_package_path_from_index(
    package_name: &str,
    package_version: &str,
    rotriever_index_path: &Path,
) -> Option<PathBuf> {
    let package_path = rotriever_index_path.join(package_name);

    // Sometimes we'll get lucky and the package will match its name exactly
    if package_path.exists() {
        return Some(package_path);
    }

    let full_package_path =
        rotriever_index_path.join(format!("{}-*-{}", package_name, package_version));

    let package_path_pattern = full_package_path
        .to_str()
        .expect("Failed to convert path to string");

    for entry in glob(&package_path_pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                if path.is_dir() {
                    return Some(path);
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }

    None
}

fn get_package(
    package_name: &str,
    package_version: Option<&str>,
    rotriever_lockfile: &RotrieverLockfile,
) -> Option<RotrieverLockfilePackage> {
    for package in &rotriever_lockfile.package {
        if package.name == package_name {
            if let Some(version) = package_version {
                if package.version != version {
                    continue;
                }
            }
            return Some(package.clone());
        }
    }
    None
}

// Rotriever stores package dependencies as "<consumer_name> <package_name>
// <version> <source>". This function parses that and finds the actual
// package in the lockfile
fn get_package_from_dependency_string(
    dependency_string: String,
    rotriever_lockfile: &RotrieverLockfile,
) -> Option<RotrieverLockfilePackage> {
    let parts = dependency_string.split(" ").collect::<Vec<&str>>();
    if parts.len() < 4 {
        return None;
    }

    let package_name = parts[1];
    let package_version = parts[2];

    get_package(package_name, Some(package_version), rotriever_lockfile)
}

fn get_packages_to_keep(package_names: &Vec<String>, dest_path: &Path) -> Vec<PathBuf> {
    let lockfile_content =
        fs::read_to_string(dest_path.join(LOCKFILE_NAME)).expect("Failed to read rotriever.lock");
    let rotriever_lockfile: RotrieverLockfile =
        toml::from_str(&lockfile_content).expect("Failed to parse rotriever.lock as TOML");

    let rotriever_index_path = dest_path.join("Packages/_Index");

    let mut packages_to_keep: Vec<PathBuf> = vec![];

    // The goal is to know which folders to keep and which ones to prune

    // TODO: Don't add duplicates

    fn process(
        package: &RotrieverLockfilePackage,
        rotriever_lockfile: &RotrieverLockfile,
        rotriever_index_path: &Path,
        packages_to_keep: &mut Vec<PathBuf>,
    ) {
        let package_dependencies = package.dependencies.clone().unwrap_or(vec![]);

        if let Some(root_dependency_path) =
            get_package_path_from_index(&package.name, &package.version, &rotriever_index_path)
        {
            if packages_to_keep.contains(&root_dependency_path) {
                info!("already processed {}, skipping", package.name);
                return;
            }

            info!(
                "found source for {} at {}",
                package.name,
                root_dependency_path.display()
            );
            packages_to_keep.push(root_dependency_path);
        }

        for dependency in package_dependencies {
            if let Some(dependency_package) =
                get_package_from_dependency_string(dependency, &rotriever_lockfile)
            {
                process(
                    &dependency_package,
                    &rotriever_lockfile,
                    &rotriever_index_path,
                    packages_to_keep,
                );
            }
        }
    }

    for package_name in package_names {
        if let Some(package) = get_package(package_name, None, &rotriever_lockfile) {
            info!("processing top-level package: {}", package.name);
            process(
                &package,
                &rotriever_lockfile,
                &rotriever_index_path,
                &mut packages_to_keep,
            );
        }
    }

    packages_to_keep
}

pub fn prune_unused_dependencies(package_names: &Vec<String>, dest_path: &Path) {
    info!("root packages to keep: {:?}", package_names);
    let packages_to_keep = get_packages_to_keep(package_names, dest_path);

    debug!("packages to keep: {:?}", packages_to_keep);

    let packages_path = dest_path.join("Packages");
    let package_index_path = packages_path.join("_Index");

    let packages_paths = fs::read_dir(&packages_path)
        .expect(format!("Failed to read {}", packages_path.display()).as_str());

    for entry in packages_paths {
        let entry = entry.expect(
            format!(
                "Failed to read directory entry in {}",
                packages_path.display()
            )
            .as_str(),
        );

        let mut should_remove = true;

        if entry.path() == package_index_path {
            continue;
        }

        for package_name_to_keep in package_names {
            if entry.file_name().to_string_lossy() == format!("{}.lua", package_name_to_keep) {
                info!("keeping {}", entry.path().display());
                should_remove = false;
                continue;
            }
        }

        if should_remove {
            info!("removing unused package: {}", entry.path().display());
            if entry.path().is_dir() {
                fs::remove_dir_all(entry.path()).expect("Failed to remove directory");
            } else {
                fs::remove_file(entry.path()).expect("Failed to remove file");
            }
        }
    }

    // Prune anything not in packages_to_keep from Packages/_Index
    let package_index_paths = fs::read_dir(&package_index_path)
        .expect(format!("Failed to read {}", package_index_path.display()).as_str());

    for entry in package_index_paths {
        let entry = entry.expect(
            format!(
                "Failed to read directory entry in {}",
                package_index_path.display()
            )
            .as_str(),
        );

        let mut should_remove = true;

        for package_path_to_keep in &packages_to_keep {
            if entry.path() == *package_path_to_keep {
                info!("keeping {}", entry.path().display());
                should_remove = false;
                continue;
            }
        }

        if should_remove {
            info!("removing unused package index: {}", entry.path().display());
            if entry.path().is_dir() {
                fs::remove_dir_all(entry.path()).expect("Failed to remove directory");
            } else {
                fs::remove_file(entry.path()).expect("Failed to remove file");
            }
        }
    }
}
