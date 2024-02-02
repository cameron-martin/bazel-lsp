use std::{
    borrow::Cow,
    io,
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
};

use ring::digest;

use crate::client::BazelInfo;

pub struct BazelWorkspace {
    pub root: PathBuf,
    /// The output base to use for querying. This allows queries to not
    /// be blocked by concurrent builds.
    pub query_output_base: Option<PathBuf>,
    pub workspace_name: Option<String>,
    pub external_output_base: PathBuf,
}

const DEFAULT_WORKSPACE_NAME: &'static str = "__main__";

impl BazelWorkspace {
    pub fn from_bazel_info<P: AsRef<Path>>(
        info: BazelInfo,
        query_output_base: Option<P>,
    ) -> io::Result<Self> {
        Ok(Self {
            root: PathBuf::from(info.workspace),
            workspace_name: PathBuf::from(info.execution_root)
                .file_name()
                .and_then(|name| match name.to_string_lossy().to_string() {
                    name if name == DEFAULT_WORKSPACE_NAME => None,
                    name => Some(name),
                }),
            external_output_base: PathBuf::from(info.output_base).join("external"),
            query_output_base: if let Some(output_base) = query_output_base {
                let hash =
                    digest::digest(&digest::SHA256, output_base.as_ref().as_os_str().as_bytes());
                let hash_hex = hex::encode(&hash);
                Some(output_base.as_ref().join(hash_hex))
            } else {
                None
            },
        })
    }

    pub fn get_repository_for_path<'a>(
        &'a self,
        path: &'a Path,
    ) -> Option<(Cow<'a, str>, &'a Path)> {
        path.strip_prefix(&self.external_output_base)
            .ok()
            .and_then(|path| {
                let mut path_components = path.components();

                let repository_name = path_components.next()?.as_os_str().to_string_lossy();
                let repository_path = path_components.as_path();

                Some((repository_name, repository_path))
            })
    }

    pub fn get_repository_path(&self, repository_name: &str) -> PathBuf {
        self.external_output_base.join(repository_name)
    }

    pub fn get_repository_names(&self) -> Vec<Cow<str>> {
        let mut names = Vec::new();
        if let Some(workspace_name) = &self.workspace_name {
            names.push(Cow::Borrowed(workspace_name.as_str()));
        }

        // Look for existing folders in `external_output_base`.
        if let Ok(entries) = std::fs::read_dir(&self.external_output_base) {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        if let Some(name) = entry.file_name().to_str() {
                            names.push(Cow::Owned(name.to_owned()));
                        }
                    }
                }
            }
        }

        names
    }
}
