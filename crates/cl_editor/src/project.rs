//! On-disk project representation.
//!
//! A project is a directory with a `project.cls` JSON file at the root and
//! sub-directories for scenes/, images/, audio/ and models/.

use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub version: String,
    #[serde(skip)]
    pub root_path: PathBuf,
}

impl Project {
    /// Create a fresh project at `path`. Scaffolds the standard
    /// sub-directories and writes a default `project.cls` manifest.
    pub fn create(path: &Path, name: &str) -> io::Result<Self> {
        fs::create_dir_all(path)?;
        let project = Self {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            root_path: path.to_path_buf(),
        };
        for sub in &["scenes", "images", "audio", "models"] {
            fs::create_dir_all(path.join(sub))?;
        }
        project.save()?;
        Ok(project)
    }

    /// Open an existing project. `path` may either point at a directory
    /// containing `project.cls` or directly at the manifest file.
    pub fn open(path: &Path) -> io::Result<Self> {
        let manifest = if path.is_dir() {
            path.join("project.cls")
        } else {
            path.to_path_buf()
        };
        let bytes = fs::read(&manifest)?;
        let mut project: Project = serde_json::from_slice(&bytes)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        project.root_path = manifest
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."));
        Ok(project)
    }

    /// Persist the project manifest to disk.
    pub fn save(&self) -> io::Result<()> {
        let manifest = self.root_path.join("project.cls");
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(manifest, json)
    }

    pub fn assets_dir(&self) -> PathBuf {
        self.root_path.clone()
    }
    pub fn images_dir(&self) -> PathBuf {
        self.root_path.join("images")
    }
    pub fn audio_dir(&self) -> PathBuf {
        self.root_path.join("audio")
    }
    pub fn models_dir(&self) -> PathBuf {
        self.root_path.join("models")
    }
    pub fn scenes_dir(&self) -> PathBuf {
        self.root_path.join("scenes")
    }

    /// List regular files inside `<root>/<sub>` (no recursion).
    pub fn list_dir(&self, sub: &str) -> Vec<PathBuf> {
        let dir = self.root_path.join(sub);
        let mut out = Vec::new();
        if let Ok(read) = fs::read_dir(&dir) {
            for entry in read.flatten() {
                let path = entry.path();
                if path.is_file() {
                    out.push(path);
                }
            }
        }
        out.sort();
        out
    }
}
