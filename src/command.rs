use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::install::install_roblox_packages;

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Install Roblox's Luau packages to a desired location
    Install {
        /// Destination where Roblox packages will be copied to
        #[clap(value_parser)]
        dest: PathBuf,

        #[clap(long, short, value_parser, num_args = 1.., value_delimiter = ' ')]
        packages: Option<Vec<String>>,
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
    pub fn run(&self) -> Result<()> {
        match &self.command {
            Command::Install { dest, packages } => {
                install_roblox_packages(dest, packages);
            }
        }

        Ok(())
    }
}
