use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
#[command(propagate_version = true)]
pub(crate) struct CliArgs {
    #[command(subcommand)]
    pub(crate) mode: Modes,
    pub(crate) path: PathBuf,
    #[arg(short)]
    pub(crate) force: bool,
    #[arg(long, default_value_t=3)]
    pub(crate) log_level: u8,
}

#[derive(Subcommand, Debug)]
#[command(author, version, about, long_about=None)]
pub(crate) enum Modes {
    #[command(subcommand)]
    Extract(ExtractFormats),
    #[command(subcommand)]
    Compress(CompressFormats),
}

#[derive(Subcommand, Debug)]
pub(crate) enum ExtractFormats {
    Zip,
    Tar,
    Auto,
}

#[derive(Subcommand, Debug)]
pub(crate) enum CompressFormats {
    Zip,
    Tar,
}
