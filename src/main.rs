use clap::Parser;
use std::env;

use crate::{
    args::{CliArgs, CompressFormats, ExtractFormats, Modes},
    tar_wrapper::{compress_tar, extract_tar},
    zip_wrapper::{compress_zip, extract_zip},
};

mod args;
mod tar_wrapper;
mod zip_wrapper;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

fn autodetect_format(args: &CliArgs) {
    let path = args.path.clone();
    let extension = path.extension().unwrap().to_str().unwrap();

    trace!("Autodetected extension as: {}", extension);

    match extension.to_lowercase().as_str() {
        "zip" => extract_zip(args),
        "tar" => extract_tar(args),
        _ => error!("Failed to detect format: {}", extension),
    }
}

fn delegate_compress(args: &CliArgs, format: &CompressFormats) {
    debug!("Mode: compress");
    match format {
        CompressFormats::Zip => compress_zip(&args).unwrap(),
        CompressFormats::Tar => compress_tar(&args),
        _ => todo!(),
    };
}

fn delegate_extract(args: &CliArgs, format: &ExtractFormats) {
    debug!("Mode: extract");
    match format {
        ExtractFormats::Zip => extract_zip(args),
        ExtractFormats::Tar => extract_tar(args),
        ExtractFormats::Auto => autodetect_format(args),
        _ => todo!(),
    }
}

fn set_log_level(args: &CliArgs) {
    match args.log_level {
        0 => env::set_var("RUST_LOG", "off"),
        1 => env::set_var("RUST_LOG", "error"),
        2 => env::set_var("RUST_LOG", "warn"),
        3 => env::set_var("RUST_LOG", "info"),
        4 => env::set_var("RUST_LOG", "debug"),
        5 => env::set_var("RUST_LOG", "trace"),
        _ => warn!("Invalid log level: {}. Should be between 0-5", args.log_level)
        
    }
}

fn main() {
    let cli_args = CliArgs::parse();
    
    #[cfg(debug_assertions)]
    env::set_var("RUST_LOG", "TRACE");
    
    set_log_level(&cli_args);

    pretty_env_logger::init();

    trace!("{:?}", cli_args);

    match cli_args.mode {
        Modes::Extract(ref format) => delegate_extract(&cli_args, &format),
        Modes::Compress(ref format) => delegate_compress(&cli_args, format),
    }
}
