use anyhow::{Context, bail};
use log::debug;
use regex::Regex;
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
    sync::LazyLock,
};

// Collection of functions for working with Rotriever's package index

// Rotriever generates a linker module for every dependency, which requires
// the dependency's folder in the index by its exact name:
//
//     local Package = require(PackageIndex["Otter-31ab8d40-1.3.7"]["Otter"])
//
// Roblox resolves packages by following these at runtime, so pruning follows
// them too. The lockfile's dependency strings have changed format between
// snapshots and don't name index folders, so they can't be trusted for this
static LINKER_TARGET_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"PackageIndex\["([^"]+)"\]"#).unwrap());

fn get_linker_targets(linker_source: &str) -> Vec<String> {
    LINKER_TARGET_PATTERN
        .captures_iter(linker_source)
        .map(|captures| captures[1].to_string())
        .collect()
}

fn get_linker_paths(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut linker_paths = vec![];

    for entry in fs::read_dir(dir).with_context(|| format!("Failed to read {}", dir.display()))? {
        let path = entry?.path();
        if path.is_file() && path.extension().is_some_and(|ext| ext == "lua") {
            linker_paths.push(path);
        }
    }

    Ok(linker_paths)
}

fn get_packages_to_keep(
    package_names: &[String],
    packages_path: &Path,
) -> anyhow::Result<HashSet<String>> {
    let package_index_path = packages_path.join("_Index");

    let mut linker_paths = vec![];
    for package_name in package_names {
        let linker_path = packages_path.join(format!("{}.lua", package_name));
        if !linker_path.is_file() {
            bail!(
                "Could not find package {} in {}",
                package_name,
                packages_path.display()
            );
        }
        linker_paths.push(linker_path);
    }

    let mut packages_to_keep = HashSet::new();

    while let Some(linker_path) = linker_paths.pop() {
        let linker_source = fs::read_to_string(&linker_path)
            .with_context(|| format!("Failed to read {}", linker_path.display()))?;

        for target in get_linker_targets(&linker_source) {
            if packages_to_keep.contains(&target) {
                continue;
            }

            let target_path = package_index_path.join(&target);
            if !target_path.is_dir() {
                bail!(
                    "{} links to {}, which is missing from the package index",
                    linker_path.display(),
                    target
                );
            }

            debug!("keeping {} (linked from {})", target, linker_path.display());
            linker_paths.extend(get_linker_paths(&target_path)?);
            packages_to_keep.insert(target);
        }
    }

    Ok(packages_to_keep)
}

fn remove_path(path: &Path) -> anyhow::Result<()> {
    if path.is_dir() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    }
    .with_context(|| format!("Failed to remove {}", path.display()))
}

pub fn prune_unused_dependencies(package_names: &[String], dest_path: &Path) -> anyhow::Result<()> {
    let packages_path = dest_path.join("Packages");
    let package_index_path = packages_path.join("_Index");

    let packages_to_keep = get_packages_to_keep(package_names, &packages_path)?;

    debug!("packages to keep: {:?}", packages_to_keep);

    let root_linkers_to_keep: HashSet<String> = package_names
        .iter()
        .map(|package_name| format!("{}.lua", package_name))
        .collect();

    for entry in fs::read_dir(&packages_path)? {
        let entry = entry?;
        let file_name = entry.file_name().to_string_lossy().to_string();

        if entry.path() == package_index_path || root_linkers_to_keep.contains(&file_name) {
            continue;
        }

        debug!("removing unused linker module: {}", entry.path().display());
        remove_path(&entry.path())?;
    }

    for entry in fs::read_dir(&package_index_path)? {
        let entry = entry?;

        if packages_to_keep.contains(&entry.file_name().to_string_lossy().to_string()) {
            continue;
        }

        debug!("removing unused package: {}", entry.path().display());
        remove_path(&entry.path())?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn linker(target: &str, name: &str) -> String {
        format!(
            "local PackageIndex = script.Parent._Index\n\
             local Package = require(PackageIndex[\"{target}\"][\"{name}\"])\n\
             return Package\n"
        )
    }

    struct Fixture {
        root: PathBuf,
    }

    impl Fixture {
        fn new(name: &str) -> Self {
            let root = std::env::temp_dir().join(format!(
                "roblox-packages-{}-{}",
                name,
                std::process::id()
            ));
            let _ = fs::remove_dir_all(&root);
            fs::create_dir_all(root.join("Packages/_Index")).unwrap();
            Self { root }
        }

        fn packages(&self) -> PathBuf {
            self.root.join("Packages")
        }

        fn add_root_linker(&self, name: &str, target: &str) {
            fs::write(
                self.packages().join(format!("{name}.lua")),
                linker(target, name),
            )
            .unwrap();
        }

        fn add_package(&self, folder: &str, name: &str, dependencies: &[(&str, &str)]) {
            let package_path = self.packages().join("_Index").join(folder);
            fs::create_dir_all(package_path.join(name)).unwrap();
            fs::write(package_path.join(name).join("init.lua"), "return {}").unwrap();

            for (alias, target) in dependencies {
                let target_name = target.split('-').next().unwrap();
                fs::write(
                    package_path.join(format!("{alias}.lua")),
                    linker(target, target_name),
                )
                .unwrap();
            }
        }

        fn index_contents(&self) -> HashSet<String> {
            fs::read_dir(self.packages().join("_Index"))
                .unwrap()
                .map(|entry| entry.unwrap().file_name().to_string_lossy().to_string())
                .collect()
        }

        fn root_contents(&self) -> HashSet<String> {
            fs::read_dir(self.packages())
                .unwrap()
                .map(|entry| entry.unwrap().file_name().to_string_lossy().to_string())
                .collect()
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }

    fn set(items: &[&str]) -> HashSet<String> {
        items.iter().map(|item| item.to_string()).collect()
    }

    #[test]
    fn reads_linker_target() {
        assert_eq!(
            get_linker_targets(&linker("Otter-31ab8d40-1.3.7", "Otter")),
            vec!["Otter-31ab8d40-1.3.7"]
        );
    }

    #[test]
    fn ignores_source_without_linker_target() {
        assert!(get_linker_targets("return {}").is_empty());
    }

    #[test]
    fn keeps_transitive_dependencies_of_requested_packages() {
        let fixture = Fixture::new("transitive");
        fixture.add_root_linker("Foundation", "Foundation");
        fixture.add_root_linker("Unused", "Unused");
        fixture.add_package("Foundation", "Foundation", &[("React", "React")]);
        fixture.add_package("React", "React", &[("Shared", "Shared")]);
        fixture.add_package("Shared", "Shared", &[]);
        fixture.add_package("Unused", "Unused", &[]);

        prune_unused_dependencies(&["Foundation".to_string()], &fixture.root).unwrap();

        assert_eq!(
            fixture.index_contents(),
            set(&["Foundation", "React", "Shared"])
        );
        assert_eq!(fixture.root_contents(), set(&["Foundation.lua", "_Index"]));
    }

    #[test]
    fn keeps_the_version_each_linker_points_at() {
        let fixture = Fixture::new("versions");
        fixture.add_root_linker("Foundation", "Foundation");
        fixture.add_package(
            "Foundation",
            "Foundation",
            &[
                ("Tokens", "Tokens-31ab8d40-6.1.1"),
                ("TokensV4", "Tokens-31ab8d40-4.0.11"),
            ],
        );
        fixture.add_package("Tokens-31ab8d40-6.1.1", "Tokens", &[]);
        fixture.add_package("Tokens-31ab8d40-4.0.11", "Tokens", &[]);
        fixture.add_package("Tokens-31ab8d40-1.0.0", "Tokens", &[]);

        prune_unused_dependencies(&["Foundation".to_string()], &fixture.root).unwrap();

        assert_eq!(
            fixture.index_contents(),
            set(&[
                "Foundation",
                "Tokens-31ab8d40-6.1.1",
                "Tokens-31ab8d40-4.0.11"
            ])
        );
    }

    #[test]
    fn handles_dependency_cycles() {
        let fixture = Fixture::new("cycles");
        fixture.add_root_linker("A", "A");
        fixture.add_package("A", "A", &[("B", "B")]);
        fixture.add_package("B", "B", &[("A", "A")]);

        prune_unused_dependencies(&["A".to_string()], &fixture.root).unwrap();

        assert_eq!(fixture.index_contents(), set(&["A", "B"]));
    }

    #[test]
    fn rejects_unknown_package() {
        let fixture = Fixture::new("unknown");
        fixture.add_root_linker("Foundation", "Foundation");
        fixture.add_package("Foundation", "Foundation", &[]);

        let result = prune_unused_dependencies(&["Fondation".to_string()], &fixture.root);

        assert!(result.is_err());
        assert_eq!(fixture.index_contents(), set(&["Foundation"]));
    }

    #[test]
    fn rejects_linker_to_missing_package() {
        let fixture = Fixture::new("missing");
        fixture.add_root_linker("Foundation", "Foundation");
        fixture.add_package("Foundation", "Foundation", &[("React", "React")]);

        let result = prune_unused_dependencies(&["Foundation".to_string()], &fixture.root);

        assert!(result.is_err());
        assert_eq!(fixture.index_contents(), set(&["Foundation"]));
    }
}
