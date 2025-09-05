use clap::Parser;
use console::style;
use log::{Level, LevelFilter, error};
use std::io::Write;

use roblox_packages::CLI;

fn main() {
    // TODO: [x] P0: Use Lune's rbx crate to find the Roblox Studio install
    // TODO: [x] P0: Extract LuaPackages
    // TODO: [ ] P0: Patch React, Roact, and RoactCompat
    // TODO: [ ] P1: Choose which packages to install, prune the rest from the index

    env_logger::Builder::from_env("LOG")
        .filter_level(LevelFilter::Info)
        .format(move |buf, record| {
            let tag = match record.level() {
                Level::Error => style("ERR").red(),
                Level::Warn => style("WARN").yellow(),
                Level::Info => style("INFO").green(),
                Level::Debug => style("DEBUG").cyan(),
                Level::Trace => style("TRACE").magenta(),
            }
            .bold();

            writeln!(buf, "{}{} {}", tag, style(":").bold(), record.args())
        })
        .init();

    let exit_code = match CLI::parse().run() {
        Ok(_) => 0,
        Err(err) => {
            error!("{:#}", err);
            1
        }
    };

    std::process::exit(exit_code)
}
