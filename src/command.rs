use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::{install::install_roblox_packages, list::list_roblox_versions};

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Install Roblox's Luau packages to a desired location
    Install {
        /// Destination where Roblox packages will be copied to
        #[clap(value_parser)]
        dest: PathBuf,

        /// The Roblox version hash to download packages from
        #[arg(short, long)]
        version: Option<String>,

        /// Optional list of package names to install (e.g. "Foundation",
        /// "Signals", "SignalsReact")
        #[arg(short, long)]
        dependencies: Option<Vec<String>>,

        /// Packages your project provides itself, as `<Name>=<require target>`.
        /// Every linker to the package is rewritten to `require(<require
        /// target>)`, and Roblox's copy is pruned along with anything only it
        /// depends on (e.g. `React=script:FindFirstAncestor("Packages").React`)
        #[arg(short, long, value_parser = parse_external, requires = "dependencies")]
        external: Vec<(String, String)>,
    },

    /// Lists the most recent versions of Roblox
    List {
        /// Limits the number of versions to print
        #[arg(long, default_value_t = 20)]
        limit: usize,
    },
}

fn parse_external(value: &str) -> Result<(String, String), String> {
    match value.split_once('=') {
        Some((name, require_target)) if !name.is_empty() && !require_target.is_empty() => {
            Ok((name.to_string(), require_target.to_string()))
        }
        _ => Err(format!(
            "expected <Name>=<require target>, got \"{}\"",
            value
        )),
    }
}

#[derive(Parser, Debug)]
#[clap(name = "roblox-packages", version, author, about)]
#[command(arg_required_else_help = true)]
pub struct CLI {
    #[clap(subcommand)]
    command: Command,
}

impl CLI {
    pub async fn run(&self) -> Result<(), anyhow::Error> {
        match &self.command {
            Command::Install {
                dest,
                version,
                dependencies,
                external,
            } => {
                let externals = external.iter().cloned().collect();
                install_roblox_packages(dest, version, dependencies, &externals).await?;
            }
            Command::List { limit } => list_roblox_versions(limit).await?,
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_external_at_first_equals() {
        assert_eq!(
            parse_external("React=Packages[\"a=b\"]"),
            Ok(("React".to_string(), "Packages[\"a=b\"]".to_string()))
        );
    }

    #[test]
    fn rejects_external_without_require_target() {
        assert!(parse_external("React").is_err());
        assert!(parse_external("React=").is_err());
        assert!(parse_external("=Packages.React").is_err());
    }
}
