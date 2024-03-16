use std::path::Path;

use starlark_lsp::server::LspUrl;

#[derive(Clone, Copy, PartialEq)]
pub enum FileType {
    Build,
    Library,
    Unknown,
}

impl FileType {
    pub const BUILD_FILE_NAMES: [&'static str; 2] = ["BUILD", "BUILD.bazel"];
    const LIBRARY_EXTENSIONS: [&'static str; 1] = ["bzl"];

    pub fn from_lsp_url(url: &LspUrl) -> Self {
        if let LspUrl::File(path) = url {
            Self::from_path(path)
        } else {
            FileType::Unknown
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        if let Some(file_name) = path.as_ref().file_name() {
            if Self::BUILD_FILE_NAMES.iter().any(|name| *name == file_name) {
                return Self::Build;
            }
        }

        if let Some(extension) = path.as_ref().extension() {
            if Self::LIBRARY_EXTENSIONS.iter().any(|ext| *ext == extension) {
                return Self::Library;
            }
        }

        FileType::Unknown
    }
}
