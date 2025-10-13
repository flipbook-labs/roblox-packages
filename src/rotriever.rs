use std::{
    fs,
    path::{Path, PathBuf},
};

use glob::glob;
use log::info;
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

fn get_packages_to_keep(package_names: &Vec<String>, dest_path: &Path) -> Vec<PathBuf> {
    let lockfile_content =
        fs::read_to_string(dest_path.join(LOCKFILE_NAME)).expect("Failed to read rotriever.lock");
    let lockfile: RotrieverLockfile =
        toml::from_str(&lockfile_content).expect("Failed to parse rotriever.lock as TOML");

    let rotriever_index_path = dest_path.join("Packages/_Index");

    let mut packages_to_keep: Vec<PathBuf> = vec![];

    // The goal is to know which folders to keep and which ones to prune

    // TODO: Bundle up this loop into a new function that returns a list of paths to keep
    for package in lockfile.package {
        if package_names.contains(&package.name) {
            let package_dependencies = package.dependencies.clone().unwrap_or(vec![]);

            let root_dependency_path =
                get_package_path_from_index(&package.name, &package.version, &rotriever_index_path);

            if root_dependency_path.is_some() {
                info!(
                    "found source for {} at {}",
                    package.name,
                    root_dependency_path.unwrap().display()
                );
            }

            for dependency in package_dependencies {
                let parts = dependency.split(" ").collect::<Vec<&str>>();
                let package_name = parts[1];
                let package_version = parts[2];

                info!(
                    "processing dependency: {} {}",
                    package_name, package_version
                );

                let dependency_path = get_package_path_from_index(
                    &package_name,
                    &package_version,
                    &rotriever_index_path,
                );

                // info!(
                //     "dependency path for {}: {:?}",
                //     package_name, dependency_path
                // );
            }
        }
    }

    // fn process_dependency(
    //     package_name: &str,
    //     packages_path: &Path,
    //     packages_to_keep: &mut Vec<String>,
    // ) {
    //     info!("keeping dependency: {}", package_name);

    //     // TODO: Handle if there are multiple versions of a package. i.e. if we
    //     // want to include Foundation as a dependency, we need to make sure we
    //     // use the right one. Maybe the most up-to-date?

    //     let dependency_path = packages_path.join(INDEX_PATH).join(package_name);

    //     let lockfile_path = dependency_path.join(LOCKFILE_NAME);
    //     let lockfile_content = fs::read_to_string(lockfile_path).expect("Failed to read lockfile");

    //     let lockfile: RotrieverLockfile =
    //         toml::from_str(&lockfile_content).expect("Failed to parse TOML");

    //     // TODO: Traverse over the dependencies to keep, use their lockfiles
    //     // to determine which _other_ dependencies to keep, and then remove
    //     // everything in the index and root of RobloxPackages that isn't in
    //     // the keep list.

    //     if let Some(deps) = lockfile.dependencies {
    //         for sub_dependency in &deps {
    //             let sub_dependency_parts: Vec<&str> = sub_dependency.split(" ").collect();
    //             let sub_package_name = sub_dependency_parts[1];

    //             info!("keeping dependency: {}", sub_package_name);

    //             packages_to_keep.push(sub_package_name.to_string());

    //             process_dependency(sub_package_name, packages_path, packages_to_keep);

    //             // TODO: Go up into the index and process the sub-dependency

    //             // TODO: Traverse over the dependencies of these
    //             // dependencies and so on until we have the full picture
    //         }
    //     }
    // }

    packages_to_keep
}

pub fn prune_unused_dependencies(package_names: &Vec<String>, dest_path: &Path) {
    info!("root packages to keep: {:?}", package_names);
    let packages_to_keep = get_packages_to_keep(package_names, dest_path);

    // TODO: Loop over both `Packages` and `Packages/_Index` and remove all
    // folders/files whose names don't match

    info!("dependencies to keep: {:?}", packages_to_keep);
}
