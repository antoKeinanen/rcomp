use std::env;
use args::CliArgs;
use clap::Parser;

use crate::{args::{Modes, Formats}, zip_wrapper::{compress_zip, extract_zip}};

mod args;
mod zip_wrapper;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

fn delegate_compress(args: &CliArgs, format: &Formats) {
    debug!("Mode: compress");
    match format {
        Formats::Zip => compress_zip(&args).unwrap(),
        _ => todo!(),
    };
}

fn delegate_extract(args: &CliArgs, format: &Formats) {
    debug!("Mode: extract");
    match format {
        Formats::Zip => extract_zip(args),
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
        Modes::Extract  {0: ref format} => delegate_extract(&cli_args, &format),
        Modes::Compress {0: ref format} => delegate_compress(&cli_args, format),
    }
}
