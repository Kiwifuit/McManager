use anyhow::Result;
use clap::Parser;

mod cmd;
#[cfg(any(feature = "forge", feature = "modrinth"))]
mod install;
mod logger;
#[cfg(feature = "packing")]
mod pack;
#[cfg(any(feature = "modrinth", feature = "forge"))]
mod tree;
mod types;

use crate::types::Args;
#[cfg(any(feature = "modrinth", feature = "forge"))]
use crate::types::Commands;

fn main() -> Result<()> {
    let global_args = Args::parse();

    let _ = logger::setup_logger(global_args.verbosity.log_level_filter());

    match global_args.subcommand {
        #[cfg(any(feature = "forge", feature = "modrinth"))]
        Commands::Info(args) => cmd::info(args)?,

        #[cfg(any(feature = "forge", feature = "modrinth"))]
        Commands::Install(args) => cmd::install(args, global_args.minecraft_home)?,

        #[cfg(any(feature = "forge", feature = "modrinth"))]
        Commands::Uninstall(args) => cmd::uninstall(args, global_args.minecraft_home)?,

        #[cfg(feature = "packing")]
        Commands::Export(args) => cmd::export(args, global_args.minecraft_home)?,
    };

    Ok(())
}
