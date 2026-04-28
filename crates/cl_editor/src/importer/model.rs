//! Wavefront OBJ import via [`tobj`].

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::project::Project;

#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub path: PathBuf,
    pub vertex_count: usize,
    pub mesh_count: usize,
}

pub fn import(project: &Project, src: &Path) -> io::Result<ModelInfo> {
    let load_opts = tobj::LoadOptions {
        single_index: true,
        triangulate: true,
        ignore_points: true,
        ignore_lines: true,
    };
    let (models, _materials) = tobj::load_obj(src, &load_opts)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

    let mesh_count = models.len();
    let vertex_count: usize = models
        .iter()
        .map(|m| m.mesh.positions.len() / 3)
        .sum();

    let dest_dir = project.models_dir();
    fs::create_dir_all(&dest_dir)?;
    let file_name = src
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "source has no file name"))?;
    let dest = dest_dir.join(file_name);
    fs::copy(src, &dest)?;

    Ok(ModelInfo {
        path: dest,
        vertex_count,
        mesh_count,
    })
}
