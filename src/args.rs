use std::path::PathBuf;

use clap::{Subcommand, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
#[command(propagate_version=true)]
pub(crate) struct CliArgs {
    #[command(subcommand)]
    pub(crate) mode: Modes,
    pub(crate) path: PathBuf,
    #[arg(short)]
    pub(crate) force: bool,
}

#[derive(Subcommand, Debug)]
#[command(author, version, about, long_about=None)]
pub(crate) enum Modes {
    #[command(subcommand)]
    Extract(Formats),
    #[command(subcommand)]
    Compress(Formats) 
}

#[derive(Subcommand, Debug)]
pub(crate) enum Formats {
    Zip,
}


