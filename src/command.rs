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
    },

    /// Lists the most recent versions of Roblox
    List {
        /// Limits the number of versions to print
        #[arg(long, default_value_t = 20)]
        limit: usize,
    },
}

#[derive(Parser, Debug)]
#[clap(name = "roblox-packages", version, author, about)]
#[command(arg_required_else_help = true)]
pub struct CLI {
    #[clap(subcommand)]
    command: Command,
}

impl CLI {
    pub async fn run(&self) -> Result<(), reqwest::Error> {
        match &self.command {
            Command::Install { dest, version } => {
                install_roblox_packages(dest, version).await?;
            }
            Command::List { limit } => list_roblox_versions(limit).await?,
        }

        Ok(())
    }
}
