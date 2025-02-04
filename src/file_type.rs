use std::path::Path;

use starlark_lsp::server::LspUrl;

#[derive(Clone, Copy, PartialEq)]
pub enum FileType {
    Build,
    Library,
    Module,
    Repo,
    Vendor,
    Workspace,
    Unknown,
}

impl FileType {
    pub const BUILD_FILE_NAMES: [&'static str; 2] = ["BUILD", "BUILD.bazel"];

    pub fn from_lsp_url(url: &LspUrl) -> Self {
        if let LspUrl::File(path) = url {
            Self::from_path(path)
        } else {
            FileType::Unknown
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        if let Some(file_name) = path.as_ref().file_name() {
            match file_name.to_string_lossy().as_ref() {
                "BUILD" | "BUILD.bazel"  => return Self::Build,
                "MODULE.bazel" => return Self::Module,
                "REPO.bazel" => return Self::Repo,
                "VENDOR.bazel" => return Self::Vendor,
                "WORKSPACE" | "WORKSPACE.bazel" => return Self::Workspace,
                _ => (),
            }
        }

        if let Some(extension) = path.as_ref().extension() {
            match extension.to_string_lossy().as_ref() {
                "bzl"  => return Self::Library,
                // It's common for repos to contain files like ext_dep.BUILD, those files are used
                // as BUILD files for external repositories.
                "BUILD"  => return Self::Build,
                _ => (),
            }
        }

        FileType::Unknown
    }
}
