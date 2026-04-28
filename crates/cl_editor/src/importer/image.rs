//! PNG / JPG image import.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::project::Project;

/// Result of a successful image import.
#[derive(Debug, Clone)]
pub struct ImageInfo {
    pub path: PathBuf,
    pub width: u32,
    pub height: u32,
    pub byte_count: u64,
}

/// Decode `src`, copy it into `<project>/images/`, and return basic metadata.
pub fn import(project: &Project, src: &Path) -> io::Result<ImageInfo> {
    let img = image::open(src).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let (width, height) = (img.width(), img.height());

    let dest_dir = project.images_dir();
    fs::create_dir_all(&dest_dir)?;
    let file_name = src
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "source has no file name"))?;
    let dest = dest_dir.join(file_name);
    fs::copy(src, &dest)?;

    let byte_count = fs::metadata(&dest)?.len();
    Ok(ImageInfo {
        path: dest,
        width,
        height,
        byte_count,
    })
}
