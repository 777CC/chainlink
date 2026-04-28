//! Asset importers — copy a file from somewhere on disk into the project's
//! per-type asset directory and return some lightweight metadata that the
//! UI can show in toasts/status bars.

pub mod audio;
pub mod image;
pub mod model;
