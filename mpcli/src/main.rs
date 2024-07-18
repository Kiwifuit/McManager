use anyhow::Result;
use clap::Parser;

mod cmd;
#[cfg(any(feature = "forge", feature = "modrinth"))]
mod install;
mod logger;
#[cfg(feature = "packing")]
mod pack;
mod tree;
mod types;

use crate::types::{Args, Commands};

fn main() -> Result<()> {
    let global_args = Args::parse();

    let _ = logger::setup_logger(global_args.verbosity.log_level_filter());

    #[cfg(any(feature = "forge", feature = "modrinth"))]
    match global_args.subcommand {
        Commands::Info(args) => cmd::info(args)?,
        Commands::Install(args) => cmd::install(args, global_args.minecraft_home)?,
        Commands::Uninstall(args) => cmd::uninstall(args, global_args.minecraft_home)?,

        #[cfg(feature = "packing")]
        Commands::Export(args) => cmd::export(args, global_args.minecraft_home)?,
    };

    Ok(())
}
