//! WAV / OGG audio import — validates the extension, then file-copies.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::project::Project;

#[derive(Debug, Clone)]
pub struct AudioInfo {
    pub path: PathBuf,
    pub byte_count: u64,
    pub ext: String,
}

pub fn import(project: &Project, src: &Path) -> io::Result<AudioInfo> {
    let ext = src
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "audio file has no extension"))?;

    if !matches!(ext.as_str(), "wav" | "ogg") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("unsupported audio extension: .{ext}"),
        ));
    }

    let dest_dir = project.audio_dir();
    fs::create_dir_all(&dest_dir)?;
    let file_name = src
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "source has no file name"))?;
    let dest = dest_dir.join(file_name);
    fs::copy(src, &dest)?;

    let byte_count = fs::metadata(&dest)?.len();
    Ok(AudioInfo {
        path: dest,
        byte_count,
        ext,
    })
}
