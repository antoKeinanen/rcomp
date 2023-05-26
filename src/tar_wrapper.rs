use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

use tar::Archive;

use crate::args::CliArgs;

pub(crate) fn extract_tar(args: &CliArgs) {
    let path = args.path.clone();
    let file = File::open(&path).unwrap();
    let mut archive = Archive::new(file);

    let mut buffer = Vec::new();
    for file in archive.entries().unwrap() {
        let mut file = file.unwrap();

        trace!(
            "{:?} ({} bytes)",
            file.header().path().unwrap(),
            file.header().size().unwrap()
        );

        let path = file.header().clone();
        let path = path.path().unwrap();

        if path.to_str().unwrap().ends_with("/") {
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
