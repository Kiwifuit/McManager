use clap::Parser;
use log::warn;

mod cmd;
mod install;
mod logger;
mod types;

use crate::types::{Args, Commands};

fn main() {
    let global_args = Args::parse();

    let _ = logger::setup_logger(global_args.verbosity.log_level_filter());
    warn!("This program is partially complete, running in 'dry run' mode");

    match global_args.subcommand {
        Commands::Info(args) => cmd::info(args),
        Commands::Install(args) => cmd::install(args, global_args.minecraft_home),
        Commands::Uninstall(args) => cmd::uninstall(args, global_args.minecraft_home),
    }
}
