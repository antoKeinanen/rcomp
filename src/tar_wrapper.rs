use std::{
    fs::{self, canonicalize, File},
    io::{Read, Write},
    path::PathBuf,
};

use tar::{Archive, Builder, EntryType};
use walkdir::WalkDir;

use crate::args::CliArgs;

pub(crate) fn extract_tar(args: &CliArgs) {
    let path = args.path.clone();
    let file = File::open(&path).unwrap();
    let mut archive = Archive::new(file);

    let dir = path.file_stem().unwrap();
    let dir = PathBuf::from(&dir);

    if dir.exists() {
        if args.force {
            info!(
                "Directory \"{}\" already exists. Overriding!",
                dir.to_str().unwrap()
            );
        } else {
            error!(
                "Directory \"{}\" already exists. Use -f flag to override.",
                dir.to_str().unwrap()
            );
            panic!();
        }
    }

    let mut buffer = Vec::new();
    for file in archive.entries().unwrap() {
        let mut file = file.unwrap();

        let path = file.header().clone();
        let path = path.path().unwrap();
        let path = dir.join(path);

        trace!("{:?} ({} bytes)", path, file.header().size().unwrap());

        if file.header().entry_type().is_dir() {
            trace!("Extracting folder to {:?}", path);
            fs::create_dir_all(&path).unwrap();
        } else {
            trace!("Extracting file to {:?}", path);
            if let Some(p) = path.parent() {
                fs::create_dir_all(p).unwrap();
            }

            file.read_to_end(&mut buffer).unwrap();
            let mut outfile = File::create(&path).unwrap();
            outfile.write_all(&buffer).unwrap();
        }
        buffer.clear();
    }
}

pub(crate) fn compress_tar(args: &CliArgs) {
    let path = args.path.clone();

    trace!("{:?} is a directory", path);
    let name = canonicalize(&path).unwrap();
    trace!("Canonicalized path: {name:?}");
    let name = name.file_stem().unwrap().to_str().unwrap();
    let name = format!("{}.tar", name);
    info!("Creating archive: {name}");

    let name = PathBuf::from(name);

    if name.exists() {
        trace!("{:?} already exists", name);
        if args.force {
            info!(
                "File \"{}\" already exists. Overriding!",
                name.to_str().unwrap()
            );
        } else {
            error!(
                "File \"{}\" already exists. Use -f flag to override.",
                name.to_str().unwrap()
            );
            panic!();
        }
    }

    let file = File::create(name).unwrap();
    let mut archive = Builder::new(file);

    let walkdir = WalkDir::new(path);

    for entry in walkdir {
        let entry = entry.unwrap();
        let path = entry.path();
        trace!("Writing {} to archive", path.display());

        if path.is_file() {
            archive
                .append_file(path, &mut File::open(path).unwrap())
                .unwrap();
        } else if path.is_dir() {
            archive.append_path(path).unwrap();
        } else {
            error!("Type of path at: {:?} is not supported!", path);
            panic!();
        }
    }
}
