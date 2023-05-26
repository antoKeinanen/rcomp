use std::{
    fs::{self, canonicalize, read_to_string, File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

use walkdir::WalkDir;
use zip::{result::ZipResult, ZipArchive};

use crate::args::CliArgs;

pub(crate) fn compress_zip(args: &CliArgs) -> ZipResult<()> {
    debug!("Format: zip");
    let path = args.path.clone();

    let name = canonicalize(&path)?;
    trace!("Canonicalized path: {name:?}");
    let name = name.file_stem().unwrap().to_str().unwrap();
    let name = format!("{}.zip", name);
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

    let walkdir = WalkDir::new(&path);

    let file = File::create(PathBuf::from(&name)).unwrap();
    let mut zip = zip::ZipWriter::new(file);

    let mut buffer = Vec::new();
    for entry in walkdir {
        let entry_path = entry.unwrap();
        let entry_path = entry_path.path();
        let name = entry_path.strip_prefix(&path).unwrap();

        if entry_path.is_file() {
            trace!("Adding file {entry_path:?} as {name:?} ...");
            #[allow(deprecated)]
            zip.start_file_from_path(name, Default::default())?;
            let mut f = File::open(&entry_path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            trace!("Adding dir {path:?} as {name:?} ...");
            #[allow(deprecated)]
            zip.add_directory_from_path(name, Default::default())?;
        }
    }

    zip.finish()?;
    Ok(())
}

pub(crate) fn extract_zip(args: &CliArgs) {
    debug!("Format: zip");
    let path = args.path.clone();

    if !path.is_file() {
        error!("{} is not a file", path.to_str().unwrap());
        panic!();
    }

    let file = File::open(&path).unwrap();
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

    let mut archive = ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => {
                warn!("Failed to find path for file at index {i}");
                continue;
            }
        };

        let outpath = dir.join(outpath);
        trace!("Outpath: {:?}", outpath);

        {
            let comment = file.comment();
            if !comment.is_empty() {
                info!("File {i} comment: {comment}");
            }
        }

        if (*file.name()).ends_with('/') {
            trace!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            trace!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
}
