use args::CliArgs;
use clap::Parser;
use std::env;

use crate::{
    args::{Formats, Modes},
    tar_wrapper::{compress_tar, extract_tar},
    zip_wrapper::{compress_zip, extract_zip},
};

mod args;
mod tar_wrapper;
mod zip_wrapper;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

fn delegate_compress(args: &CliArgs, format: &Formats) {
    debug!("Mode: compress");
    match format {
        Formats::Zip => compress_zip(&args).unwrap(),
        Formats::Tar => compress_tar(&args),
        _ => todo!(),
    };
}

fn delegate_extract(args: &CliArgs, format: &Formats) {
    debug!("Mode: extract");
    match format {
        Formats::Zip => extract_zip(args),
        Formats::Tar => extract_tar(args),
        _ => todo!(),
    }
}

fn main() {
    #[cfg(debug_assertions)]
    env::set_var("RUST_LOG", "TRACE");

    pretty_env_logger::init();
    let cli_args = CliArgs::parse();

    trace!("{:?}", cli_args);

    match cli_args.mode {
        Modes::Extract { 0: ref format } => delegate_extract(&cli_args, &format),
        Modes::Compress { 0: ref format } => delegate_compress(&cli_args, format),
    }
}
